import gymnasium as gym
import torch
from torch import nn
from experience_replay import ReplayMemory
from dqn import DQN
import random
import os
import matplotlib.pyplot as plt

device = (
    torch.accelerator.current_accelerator().type
    if torch.accelerator.is_available()
    else "cpu"
)


class Agent:
    def __init__(self):
        self.MODEL_FILE = os.path.join("runs", "cartpole.pt")
        self.env = gym.make("CartPole-v1")
        self.n_observations = self.env.observation_space.shape[0]
        self.n_hidden = 256
        self.n_actions = self.env.action_space.n
        self.policy_net = DQN(self.n_observations, self.n_hidden, self.n_actions).to(
            device
        )

        self.loss_fn = nn.MSELoss()
        self.optimizer = None

        self.mini_batch_size = 64
        self.discount_factor_g = 0.99
        self.network_sync_rate = 100
        self.epsilon = 1
        self.epsilon_decay = 0.9995
        self.epsilon_min = 0.05
        self.learning_rate_a = 0.001
        self.max_episode_reward = 10_000

    def train(self):
        env = self.env
        target_net = DQN(self.n_observations, self.n_hidden, self.n_actions).to(device)
        target_net.load_state_dict(self.policy_net.state_dict())
        self.optimizer = torch.optim.Adam(
            self.policy_net.parameters(), lr=self.learning_rate_a
        )

        rewards_per_episode = []
        step_count = 0
        memory = ReplayMemory(10_000)
        for episode in range(10_000):
            state, _ = env.reset()
            state = torch.tensor(state, dtype=torch.float, device=device)
            terminated = False
            episode_reward = 0

            while not terminated and episode_reward < self.max_episode_reward:
                if random.random() < self.epsilon:
                    action = env.action_space.sample()
                    action = torch.tensor(action, dtype=torch.int64, device=device)
                else:
                    with torch.no_grad():
                        action = self.policy_net(state.unsqueeze(dim=0)).argmax()

                next_state, reward, terminated, _, info = env.step(action.item())
                next_state = torch.tensor(next_state, dtype=torch.float, device=device)
                reward = torch.tensor(reward, dtype=torch.float, device=device)
                memory.append((state, action, next_state, reward, terminated))

                state = next_state
                episode_reward += reward
                self.epsilon = max(self.epsilon * self.epsilon_decay, self.epsilon_min)
            print(episode_reward.item())
            rewards_per_episode.append(episode_reward.item())

            step_count += 1
            if len(memory) > self.mini_batch_size:
                mini_batch = memory.sample(self.mini_batch_size)
                self.optimize(mini_batch, target_net)

                if step_count > self.network_sync_rate:
                    target_net.load_state_dict(self.policy_net.state_dict())
                    step_count = 0

        torch.save(self.policy_net.state_dict(), self.MODEL_FILE)
        self.graph(rewards_per_episode)

    def optimize(self, mini_batch, target_net):
        states, actions, next_states, rewards, terminations = zip(*mini_batch)
        states = torch.stack(states)
        actions = torch.stack(actions)
        next_states = torch.stack(next_states)
        rewards = torch.stack(rewards)
        terminations = torch.tensor(terminations).float().to(device)

        with torch.no_grad():
            target_qs = (
                rewards
                + (1 - terminations)
                * self.discount_factor_g
                * target_net(next_states).max(dim=1)[0]
            )
        current_qs = (
            self.policy_net(states)
            .gather(dim=1, index=actions.unsqueeze(dim=1))
            .squeeze()
        )

        loss = self.loss_fn(current_qs, target_qs)
        self.optimizer.zero_grad()
        loss.backward()
        self.optimizer.step()

        # for state, action, next_state, reward, terminated in mini_batch:
        #     if terminated:
        #         target_q = reward
        #     else:
        #         with torch.no_grad():
        #             target_q = (
        #                 reward * self.discount_factor_g * target_net(next_state).max()
        #             )
        #
        #         loss = self.loss_fn(current_q, target_q)
        #         self.optimizer.zero_grad()
        #         loss.backward()
        #         self.optimizer.step()

    def graph(self, rewards_per_episode):
        # mean_rewards = np.zeros(len(rewards_per_episode))
        # for x in len(rewards_per_episode)
        # for reward_per_episode in rewards_per_episode:
        plt.plot(rewards_per_episode)
        plt.savefig("rewards_plot.png")
        plt.close()

    def test(self):
        self.policy_net.load_state_dict(torch.load(self.MODEL_FILE))
        self.policy_net.eval()
        env = gym.make("CartPole-v1", render_mode="human")
        state, _ = env.reset()
        state = torch.tensor(state, dtype=torch.float, device=device)
        terminated = False
        while not terminated:
            with torch.no_grad():
                action = self.policy_net(state.unsqueeze(dim=0)).argmax()
            state, _, terminated, _, info = env.step(action.item())
            state = torch.tensor(state, dtype=torch.float, device=device)
            env.render()
