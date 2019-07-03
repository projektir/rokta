use std::io::{stdin, stdout};

use game::Game;

mod art;
mod game;

fn main() {
    let stdin = stdin();
    let stdout = stdout();

    let mut game = Game::new(stdin.lock(), stdout.lock());
    game.run();
}
