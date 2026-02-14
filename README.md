# neat-snake

snake ai, environment written in rust

This project uses maturin (for now)
to use maturin with uv, run

```shell
uv tool install maturin
```

inside the snake-rust folder run

```shell
# debug build
maturin develop
# release builds
maturin develop -r
```

and then you can just run

```shell
uv run main.py
```

in the root folder
