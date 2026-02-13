use std::io;

use crate::snake::{Game, GameState};
mod snake;
fn main() {
    let mut game = Game::new();
    game.render();
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

        if let GameState::Finished = game.step(dir_x, dir_y) {
            return;
            // return self.score;
        }
        game.render();
    }
}
