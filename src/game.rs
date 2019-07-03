use std::io::{Read, Write};

use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor, style};

use super::game_loop::welcome_loop;

pub struct Game<R, W: Write> {
    pub stdout: W,
    pub stdin: R,
}

impl<R: Read, W: Write> Game<R, W> {
    pub fn new(stdin: R, stdout: W) -> Game<R, RawTerminal<W>> {
        Game {
            stdin,
            stdout: stdout.into_raw_mode().unwrap(),
        }
    }

    fn init(&mut self) {
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Hide
        )
        .unwrap();
    }

    pub fn run(&mut self) {
        self.init();

        let quit = welcome_loop(&mut *self);
        if quit {
            return;
        }

        loop {
            let mut b = [0];
            self.stdin.read(&mut b).unwrap();

            match b[0] {
                b'q' => return,
                _ => {}
            }

            self.stdout.flush().unwrap();
        }
    }
}

impl<R, W: Write> Drop for Game<R, W> {
    fn drop(&mut self) {
        write!(
            self.stdout,
            "{}{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(1, 1),
            cursor::Show
        )
        .unwrap();
    }
}
