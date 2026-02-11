use std::io;

use rand::{RngExt, rngs::ThreadRng};

const SNAKE_GRID_SIZE: usize = 4;

#[derive(PartialEq, Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

fn gen_rand_apple(
    rng: &mut ThreadRng,
    snake_grid: [[usize; SNAKE_GRID_SIZE]; SNAKE_GRID_SIZE],
    snake_length: usize,
) -> Position {
    let len_available_cells = SNAKE_GRID_SIZE.pow(2) - snake_length;
    let random_apple_i = rng.random_range(0..len_available_cells);
    let mut i = 0;
    for (y, snake_grid_row) in snake_grid.iter().enumerate().take(SNAKE_GRID_SIZE) {
        for (x, snake_grid_cell) in snake_grid_row.iter().enumerate().take(SNAKE_GRID_SIZE) {
            if *snake_grid_cell != 0 {
                continue;
            }
            if i == random_apple_i {
                return Position::new(x, y);
            } else {
                i += 1;
            }
        }
    }
    panic!("no free cell for apple, code bug");
}

fn main() {
    let mut rng = rand::rng();
    let mut snake_grid = [[0; SNAKE_GRID_SIZE]; SNAKE_GRID_SIZE];
    // let mut snake_cells = Vec::with_capacity(SNAKE_GRID_SIZE.pow(2));
    // snake_cells.push(Position::new(0, 0));
    let mut snake_cells = [Position::new(0, 0); SNAKE_GRID_SIZE.pow(2)];
    let mut snake_length = 1;
    let mut gen_apple = false;
    let mut rand_apple = gen_rand_apple(&mut rng, snake_grid, snake_length);
    snake_grid[rand_apple.y][rand_apple.x] = 1;

    'game: loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let (dir_x, dir_y): (isize, isize) = match input.trim() {
            "w" => (0, -1),
            "a" => (-1, 0),
            "s" => (0, 1),
            "d" => (1, 0),
            _ => (0, 0),
        };

        let snake_head = snake_cells[0];

        let new_head_x = snake_head.x as isize + dir_x;
        let new_head_y = snake_head.y as isize + dir_y;

        let new_head_x: usize = new_head_x.clamp(0, SNAKE_GRID_SIZE as isize - 1) as usize;
        let new_head_y: usize = new_head_y.clamp(0, SNAKE_GRID_SIZE as isize - 1) as usize;

        if snake_head.x == new_head_x && snake_head.y == new_head_y {
            continue;
        }

        for snake_cell in snake_cells.iter().take(snake_length - 1).skip(1) {
            if snake_cell.x == new_head_x && snake_cell.y == new_head_y {
                continue 'game;
            }
        }

        if new_head_x == rand_apple.x && new_head_y == rand_apple.y {
            snake_length += 1;
            if snake_length == SNAKE_GRID_SIZE.pow(2) {
                println!("Game won");
                return;
            }
            gen_apple = true;
        }

        let snake_tail = snake_cells[snake_length - 1];
        snake_grid[snake_tail.y][snake_tail.x] = 0;
        for snake_i in (1..snake_length).rev() {
            let prev_snake_cell = snake_cells[snake_i - 1];
            dbg!("{:?}", prev_snake_cell);
            snake_grid[prev_snake_cell.y][prev_snake_cell.x] = snake_i + 18;
            snake_cells[snake_i] = prev_snake_cell;
        }

        let snake_head = &mut snake_cells[0];
        snake_head.x = new_head_x;
        snake_head.y = new_head_y;
        snake_grid[snake_head.y][snake_head.x] = 2;

        if gen_apple {
            rand_apple = gen_rand_apple(&mut rng, snake_grid, snake_length);
            snake_grid[rand_apple.y][rand_apple.x] = 1;
            gen_apple = false;
        }

        dbg!(snake_length);
        println!("\x1Bc");
        for snake_row in snake_grid {
            println!("{:?}", snake_row);
        }
    }
}
