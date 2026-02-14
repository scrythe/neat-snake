use pyo3::prelude::*;
use rand::{RngExt, SeedableRng, rngs::ThreadRng};
use rand_chacha::ChaCha8Rng;

const SNAKE_GRID_SIZE: usize = 8;
const SNAKE_GREEN_COLORS_AMOUNT: usize = 255 / SNAKE_GRID_SIZE.pow(2);
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
    rng: &mut ChaCha8Rng,
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

#[pyclass(eq)]
#[derive(PartialEq)]
pub enum GameState {
    Continue,
    Finished,
}

#[pyclass]
pub struct Game {
    rng: ChaCha8Rng,
    snake_grid: [[f32; SNAKE_GRID_SIZE]; SNAKE_GRID_SIZE],
    snake_body_cells: [Position; SNAKE_GRID_SIZE.pow(2)],
    snake_length: usize,
    rand_apple_pos: Position,
    remaining_steps: i32,
    steps_away_apple: i32,
    score: i32,
}

#[pymethods]
impl Game {
    #[new]
    #[pyo3(signature = (seed=None))]
    pub fn new(seed: Option<u64>) -> Game {
        let mut rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_rng(&mut rand::rng()),
        };
        // rng.set_stream(0);
        let mut snake_grid = [[0.0; SNAKE_GRID_SIZE]; SNAKE_GRID_SIZE];
        let rand_snake_head_pos = Position::new(
            rng.random_range(0..SNAKE_GRID_SIZE),
            rng.random_range(0..SNAKE_GRID_SIZE),
        );
        snake_grid[rand_snake_head_pos.y][rand_snake_head_pos.x] = 2.0;
        let snake_body_cells = [rand_snake_head_pos; SNAKE_GRID_SIZE.pow(2)];
        let snake_length = 1;
        let rand_apple_pos = gen_rand_apple(&mut rng, snake_grid, snake_length);
        snake_grid[rand_apple_pos.y][rand_apple_pos.x] = 1.0;
        let remaining_steps = STEPS_TILL_APPLE;
        let steps_away_apple = 0;
        let score = 0;

        Game {
            rng,
            snake_grid,
            snake_body_cells,
            snake_length,
            rand_apple_pos,
            remaining_steps,
            steps_away_apple,
            score,
        }
    }

    pub fn reset(&mut self, seed: Option<u64>) {
        match seed {
            Some(seed) => self.rng = ChaCha8Rng::seed_from_u64(seed),
            None => self.rng = ChaCha8Rng::from_rng(&mut rand::rng()),
        }
        self.snake_grid = [[0.0; SNAKE_GRID_SIZE]; SNAKE_GRID_SIZE];
        let rand_snake_head_pos = Position::new(
            self.rng.random_range(0..SNAKE_GRID_SIZE),
            self.rng.random_range(0..SNAKE_GRID_SIZE),
        );
        self.snake_grid[rand_snake_head_pos.y][rand_snake_head_pos.x] = 2.0;
        self.snake_body_cells = [rand_snake_head_pos; SNAKE_GRID_SIZE.pow(2)];
        self.snake_length = 1;
        self.rand_apple_pos = gen_rand_apple(&mut self.rng, self.snake_grid, self.snake_length);
        self.snake_grid[self.rand_apple_pos.y][self.rand_apple_pos.x] = 1.0;
        self.remaining_steps = STEPS_TILL_APPLE;
        self.steps_away_apple = 0;
        self.score = 0;
    }

    pub fn step(&mut self, dir_x: isize, dir_y: isize) -> GameState {
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

        // if you can go to position that will be free, than instead take(self.snake_length -1) and
        // below comented code for edge case when length 2
        for snake_cell in self.snake_body_cells.iter().take(self.snake_length).skip(1) {
            if snake_cell.x == new_head_x && snake_cell.y == new_head_y {
                return GameState::Continue;
            }
        }
        // if self.snake_length == 2
        //     && self.snake_body_cells[1].x == new_head_x
        //     && self.snake_body_cells[1].y == new_head_y
        // {
        //     return GameState::Continue;
        // }

        let is_new_x_closer = self.rand_apple_pos.x.abs_diff(new_head_x)
            < self.rand_apple_pos.x.abs_diff(snake_head.x);
        let is_new_y_closer = self.rand_apple_pos.y.abs_diff(new_head_y)
            < self.rand_apple_pos.y.abs_diff(snake_head.y);
        if is_new_x_closer || is_new_y_closer {
            self.score += STEP_TOWARDS_APPLE_BONUS;
        } else {
            self.steps_away_apple += 1
        }

        let snake_ate_apple =
            new_head_x == self.rand_apple_pos.x && new_head_y == self.rand_apple_pos.y;
        if snake_ate_apple {
            self.snake_length += 1;
            self.score +=
                (GETTING_APPLE_BONUS - STEP_AWAY_APPLE_BONUS * self.steps_away_apple).max(0);
            if self.snake_length == SNAKE_GRID_SIZE.pow(2) {
                return GameState::Finished;
            }
            self.remaining_steps = STEPS_TILL_APPLE;
            self.steps_away_apple = 0;
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

        if snake_ate_apple {
            self.rand_apple_pos = gen_rand_apple(&mut self.rng, self.snake_grid, self.snake_length);
            self.snake_grid[self.rand_apple_pos.y][self.rand_apple_pos.x] = 1.0;
        }
        GameState::Continue
    }

    pub fn render(&mut self) {
        println!("\x1Bc");
        for snake_grid_row in self.snake_grid {
            for snake_grid_cell in snake_grid_row {
                match snake_grid_cell {
                    0.0 => print!("\x1B[48;5;240m  \x1B[0m"),     // empty gray
                    1.0 => print!("\x1B[48;5;196m  \x1B[0m"),     // apple red
                    2.0 => print!("\x1B[48;2;0;255;0m  \x1B[0m"), // head green
                    snake_body_i => {
                        let mut snake_body_i_no_offset = snake_body_i as usize - 19;
                        // snake body after head ranges from 0 to 20, if larger it goes backwards
                        // so 21 becomes 19, and if it does that twice like when its larger than
                        // 40, than it goes around normally, so just below if statements
                        if snake_body_i_no_offset >= 40 {
                            snake_body_i_no_offset %= 40;
                        }
                        if snake_body_i_no_offset > 20 {
                            snake_body_i_no_offset = 20 - snake_body_i_no_offset % 20;
                        }
                        let green_color_strength = 200 - snake_body_i_no_offset % 21 * 10;
                        print!("\x1B[48;2;0;{green_color_strength};0m  \x1B[0m");
                    }
                }
            }
            println!()
        }
    }
}
