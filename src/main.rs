use rand::RngExt;

const SNAKE_GRID_SIZE: usize = 4;

#[derive(PartialEq)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

fn gen_rand_apple(snake_pos: &Position) -> Position {
    let mut rng = rand::rng();
    loop {
        let x = rng.random_range(0..SNAKE_GRID_SIZE);
        let y = rng.random_range(0..SNAKE_GRID_SIZE);
        if snake_pos.x != x && snake_pos.y != y {
            return Position::new(x, y);
        }
    }
}

fn main() {
    let mut snake_grid = [[0; SNAKE_GRID_SIZE]; SNAKE_GRID_SIZE];
    let snake_pos = Position::new(0, 0);
    let apple_pos = gen_rand_apple(&snake_pos);

    snake_grid[snake_pos.y][snake_pos.x] = 2;
    snake_grid[apple_pos.y][apple_pos.x] = 1;

    for snake_row in snake_grid {
        println!("{:?}", snake_row);
    }
}
