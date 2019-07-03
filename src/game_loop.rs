use std::io::{Read, Write};

use super::art::{draw_ship, draw_welcome_screen};
use super::game::Game;

pub fn welcome_loop<R: Read, W: Write>(game: &mut Game<R, W>) -> bool {
    draw_welcome_screen(&mut *game);

    loop {
        let mut b = [0];
        game.stdin.read(&mut b).unwrap();

        match b[0] {
            b' ' => {
                draw_ship(&mut *game);
                return false;
            }
            b'q' => return true,
            _ => {}
        }

        game.stdout.flush().unwrap();
    }
}
