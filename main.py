import neat_snake
import time

start = time.time()
game = neat_snake.Game(0)
game.render()
while True:
    action = input()
    key_map = {
        "w": (0, -1),
        "a": (-1, 0),
        "s": (0, 1),
        "d": (1, 0),
    }
    dir_x, dir_y = key_map.get(action, (0, 0))
    state = game.step(dir_x, dir_y)
    game.render()
    if state == neat_snake.GameState.Finished:
        break
