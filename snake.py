import snake_rust


def main():
    game = snake_rust.Game(0)
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
        if state == snake_rust.GameState.Finished:
            break


if __name__ == "__main__":
    main()
