use std::io::{Read, Write};

use termion::{clear, color, style};

use super::color::{LIGHT_STEEL_BLUE, STEEL_BLUE};
use super::game::Game;

const BOX_HOR: &'static str = "+======================+\n\r";

pub fn draw_welcome_screen<R: Read, W: Write>(game: &mut Game<R, W>) {
    reset_draw(game);

    write!(game.stdout, "{}{}", color::Fg(color::LightBlue), BOX_HOR).unwrap();
    write!(
        game.stdout,
        "| {}q        quit{}        |\n\r",
        color::Fg(color::LightWhite),
        color::Fg(color::LightBlue)
    )
    .unwrap();
    write!(
        game.stdout,
        "| {}space    start{}       |\n\r",
        color::Fg(color::LightWhite),
        color::Fg(color::LightBlue)
    )
    .unwrap();
    write!(game.stdout, "{}{}", color::Fg(color::LightBlue), BOX_HOR).unwrap();

    game.stdout.flush().unwrap();
}

pub fn draw_ship<R: Read, W: Write>(game: &mut Game<R, W>) {
    reset_draw(game);

    write!(game.stdout, "{}   \\\\\n\r", color::Fg(LIGHT_STEEL_BLUE)).unwrap();
    write!(
        game.stdout,
        "  {}##{}[]{}>\n\r",
        color::Fg(color::Rgb(255, 140, 0)),
        color::Fg(STEEL_BLUE),
        color::Fg(LIGHT_STEEL_BLUE)
    )
    .unwrap();
    write!(game.stdout, "{}   //\n\r", color::Fg(LIGHT_STEEL_BLUE)).unwrap();

    game.stdout.flush().unwrap();
}

fn reset_draw<R: Read, W: Write>(game: &mut Game<R, W>) {
    write!(game.stdout, "{}{}", clear::All, style::Reset).unwrap();

    game.stdout.flush().unwrap();
}
