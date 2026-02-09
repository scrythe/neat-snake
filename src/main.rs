use std::io;

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

// enum Direction {
//    W(String::from("w"))
// }

fn main() {
    let mut snake_grid = [[0; SNAKE_GRID_SIZE]; SNAKE_GRID_SIZE];
    let mut snake_pos = Position::new(0, 0);
    let mut rand_apple_pos = gen_rand_apple(&snake_pos);

    snake_grid[snake_pos.y][snake_pos.x] = 2;
    snake_grid[rand_apple_pos.y][rand_apple_pos.x] = 1;

    println!("\x1Bc");
    for snake_row in snake_grid {
        println!("{:?}", snake_row);
    }
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let (new_x, new_y) = match input.trim() {
            "w" => (snake_pos.x, snake_pos.y.saturating_sub(1)),
            "a" => (snake_pos.x.saturating_sub(1), snake_pos.y),
            "s" => (snake_pos.x, (snake_pos.y + 1).min(SNAKE_GRID_SIZE - 1)),
            "d" => ((snake_pos.x + 1).min(SNAKE_GRID_SIZE - 1), snake_pos.y),
            _ => (snake_pos.x, snake_pos.y),
        };
        let snake_eats_apple = new_x == rand_apple_pos.x && new_y == rand_apple_pos.y;
        if snake_eats_apple {
            snake_pos.x = new_x;
            snake_pos.y = new_y;
            rand_apple_pos = gen_rand_apple(&snake_pos);
            snake_grid[rand_apple_pos.y][rand_apple_pos.x] = 1;
        } else {
            snake_grid[snake_pos.y][snake_pos.x] = 0;
            snake_pos.x = new_x;
            snake_pos.y = new_y;
        }
        snake_grid[snake_pos.y][snake_pos.x] = 2;

        println!("\x1Bc");
        for snake_row in snake_grid {
            println!("{:?}", snake_row);
        }
    }
}
