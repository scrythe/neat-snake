# look at readme but heavily inspired from johnnycode
# https://github.com/johnnycode8/dqn_pytorch?tab=MIT-1-ov-file#readme

from torch import nn
from torch.nn import functional as F


class DQN(nn.Module):
    def __init__(self, n_observations: int, n_hidden: int, n_actions: int):
        super().__init__()
        self.layer1 = nn.Linear(n_observations, n_hidden)
        self.layer2 = nn.Linear(n_hidden, n_actions)

    def forward(self, x):
        x = F.relu(self.layer1(x))
        return self.layer2(x)
