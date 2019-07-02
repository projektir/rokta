use termion::{color, style};

fn main() {
    println!("{}", color::Fg(color::Red));
    draw_horizontal();

    for i in 0 .. 25 {
        draw_vertical();
    }

    draw_horizontal();

    print!("{}{}", termion::clear::All, termion::cursor::Goto(5, 5));
}

fn draw_horizontal() {
    println!("------------------------------------------------------------");
}

fn draw_vertical() {
    println!("|                                                          |");
}
