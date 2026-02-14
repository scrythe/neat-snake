use std::io;

mod snake;
use crate::snake::{Game, GameState};

fn game_loop(game: &mut Game) {
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

fn main() {
    let mut game = Game::new(Some(0));
    game.render();
    game_loop(&mut game);
    game.reset(Some(0));
    game.render();
    game_loop(&mut game);
}
