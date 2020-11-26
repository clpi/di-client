use std::{
    io::{stdin, stdout, Write},
    time::Duration, thread
};
use termion::{
    input::TermRead,
    raw::IntoRawMode,
    event::Key,
    clear, cursor,
    color,
};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}q to exit{}",
        clear::All, cursor::Goto(1, 1), cursor::Hide)
        .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout, "{}{}", cursor::Goto(1, 1), clear::CurrentLine)
            .unwrap();
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("Alt-{}", c),
            Key::Ctrl(c) => println!("Ctrl-{}", c),
            Key::Left => println!("<left>"),
            Key::Up => println!("<up>"),
            Key::Right => println!("<right>"),
            Key::Down => println!("<down>"),
            _ => println!("other")
        }
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", cursor::Show).unwrap();
}
