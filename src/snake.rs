use std::io;

use radiate::{Eval, Graph, Op, genotype};
use rand::{RngExt, rngs::ThreadRng};

const SNAKE_GRID_SIZE: usize = 8;
const STEPS_TILL_APPLE: i32 = 50;
const GETTING_APPLE_BONUS: i32 = 1000;
const STEP_TOWARDS_APPLE_BONUS: i32 = 10;
const STEP_AWAY_APPLE_BONUS: i32 = -15;
const WINNING_BONUS: i32 = 50;
const LOOSING_BONUS: i32 = -1000;

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
    snake_grid: [[f32; SNAKE_GRID_SIZE]; SNAKE_GRID_SIZE],
    snake_length: usize,
) -> Position {
    let len_available_cells = SNAKE_GRID_SIZE.pow(2) - snake_length;
    let random_apple_i = rng.random_range(0..len_available_cells);
    let mut i = 0;
    for (y, snake_grid_row) in snake_grid.iter().enumerate().take(SNAKE_GRID_SIZE) {
        for (x, snake_grid_cell) in snake_grid_row.iter().enumerate().take(SNAKE_GRID_SIZE) {
            if *snake_grid_cell != 0.0 {
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
enum GameState {
    Continue,
    Finished,
}

pub struct Game {
    rng: ThreadRng,
    snake_grid: [[f32; SNAKE_GRID_SIZE]; SNAKE_GRID_SIZE],
    snake_body_cells: [Position; SNAKE_GRID_SIZE.pow(2)],
    snake_length: usize,
    should_gen_apple: bool,
    rand_apple_pos: Position,
    remaining_steps: i32,
    steps_away_apple: i32,
    score: i32,
    should_print: bool,
}

impl Game {
    pub fn new() -> Game {
        let mut rng = rand::rng();
        let mut snake_grid = [[0.0; SNAKE_GRID_SIZE]; SNAKE_GRID_SIZE];
        let rand_snake_head_pos = Position::new(
            rng.random_range(0..SNAKE_GRID_SIZE),
            rng.random_range(0..SNAKE_GRID_SIZE),
        );
        let snake_body_cells = [rand_snake_head_pos; SNAKE_GRID_SIZE.pow(2)];
        let snake_length = 1;
        let should_gen_apple = false;
        let rand_apple_pos = gen_rand_apple(&mut rng, snake_grid, snake_length);
        snake_grid[rand_apple_pos.y][rand_apple_pos.x] = 1.0;
        let remaining_steps = STEPS_TILL_APPLE;
        let steps_away_apple = 0;
        let score = 0;
        let should_print = false;

        Game {
            rng,
            snake_grid,
            snake_body_cells,
            snake_length,
            should_gen_apple,
            rand_apple_pos,
            remaining_steps,
            steps_away_apple,
            score,
            should_print,
        }
    }

    pub fn print(mut self) -> Self {
        self.should_print = true;
        self
    }

    fn take_step(&mut self, dir_x: isize, dir_y: isize) -> GameState {
        if self.remaining_steps <= 0 {
            // self.score += LOOSING_BONUS;
            return GameState::Finished;
        }
        self.remaining_steps -= 1;

        let snake_head = self.snake_body_cells[0];

        let new_head_x = snake_head.x as isize + dir_x;
        let new_head_y = snake_head.y as isize + dir_y;

        if new_head_x < 0 || new_head_x >= SNAKE_GRID_SIZE as isize {
            self.score += LOOSING_BONUS;
            return GameState::Finished;
        }
        if new_head_y < 0 || new_head_y >= SNAKE_GRID_SIZE as isize {
            self.score += LOOSING_BONUS;
            return GameState::Finished;
        }

        let new_head_x = new_head_x as usize;
        let new_head_y = new_head_y as usize;

        if snake_head.x == new_head_x && snake_head.y == new_head_y {
            return GameState::Continue;
        }

        for snake_cell in self
            .snake_body_cells
            .iter()
            .take(self.snake_length - 1)
            .skip(1)
        {
            if snake_cell.x == new_head_x && snake_cell.y == new_head_y {
                return GameState::Continue;
            }
        }

        let is_new_x_closer = self.rand_apple_pos.x.abs_diff(new_head_x)
            < self.rand_apple_pos.x.abs_diff(snake_head.x);
        let is_new_y_closer = self.rand_apple_pos.y.abs_diff(new_head_y)
            < self.rand_apple_pos.y.abs_diff(snake_head.y);
        if is_new_x_closer || is_new_y_closer {
            self.score += STEP_TOWARDS_APPLE_BONUS;
        } else {
            self.steps_away_apple += 1
        }

        if new_head_x == self.rand_apple_pos.x && new_head_y == self.rand_apple_pos.y {
            self.snake_length += 1;
            self.score +=
                (GETTING_APPLE_BONUS - STEP_AWAY_APPLE_BONUS * self.steps_away_apple).max(0);
            // if self.should_print {
            //     dbg!((GETTING_APPLE_BONUS - STEP_AWAY_APPLE_BONUS * self.steps_away_apple).max(0));
            // }
            // dbg!(self.score);
            if self.snake_length == SNAKE_GRID_SIZE.pow(2) {
                // self.score += WINNING_BONUS;
                return GameState::Finished;
            }
            self.remaining_steps = 15;
            self.steps_away_apple = 0;
            self.should_gen_apple = true;
        }

        let snake_tail = self.snake_body_cells[self.snake_length - 1];
        self.snake_grid[snake_tail.y][snake_tail.x] = 0.0;
        for snake_i in (1..self.snake_length).rev() {
            let prev_snake_cell = self.snake_body_cells[snake_i - 1];
            self.snake_grid[prev_snake_cell.y][prev_snake_cell.x] = snake_i as f32 + 18.0;
            self.snake_body_cells[snake_i] = prev_snake_cell;
        }

        let snake_head = &mut self.snake_body_cells[0];
        snake_head.x = new_head_x;
        snake_head.y = new_head_y;
        self.snake_grid[snake_head.y][snake_head.x] = 2.0;

        if self.should_gen_apple {
            self.rand_apple_pos = gen_rand_apple(&mut self.rng, self.snake_grid, self.snake_length);
            self.snake_grid[self.rand_apple_pos.y][self.rand_apple_pos.x] = 1.0;
            self.should_gen_apple = false;
        }
        GameState::Continue
    }

    pub fn run_game_human(mut self) -> i32 {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let (dir_x, dir_y): (isize, isize) = match input.trim() {
                "w" => (0, -1),
                "a" => (-1, 0),
                "s" => (0, 1),
                "d" => (1, 0),
                _ => (0, 0),
            };

            if let GameState::Finished = self.take_step(dir_x, dir_y) {
                return self.score;
            }

            // println!("\x1Bc");
            for snake_row in self.snake_grid {
                println!("{:?}", snake_row);
            }
            println!("----");
        }
    }

    pub fn run_game_ai(mut self, genotype: Graph<Op<f32>>) -> i32 {
        loop {
            // let test: Vec<f32> = self.snake_grid.into_iter().flatten().collect();
            let observation = self.snake_grid.as_flattened().to_vec();

            let outputs = genotype.eval(&[observation]);
            let (max_action, _) = outputs[0]
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap();

            let (dir_x, dir_y): (isize, isize) = match max_action {
                0 => (0, -1),
                1 => (-1, 0),
                2 => (0, 1),
                3 => (1, 0),
                _ => (0, 0),
            };

            if let GameState::Finished = self.take_step(dir_x, dir_y) {
                return self.score;
            };

            if self.should_print {
                // dbg!("{}", self.score);
                // println!("\x1Bc");
                for snake_row in self.snake_grid {
                    println!("{:?}", snake_row);
                }
                println!("----");
            }
        }
    }
}

pub fn snake_game_fitness_func(genotype: Graph<Op<f32>>) -> i32 {
    let snake_game = Game::new();
    snake_game.run_game_ai(genotype)
}

fn main() {
    let snake_game = Game::new();
    let score = snake_game.run_game_human();
    println!("{}", score);
}
