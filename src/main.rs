use termion::{color, style, terminal_size};
use std::io;

fn main() {
    println!("{}{}Stuff", termion::clear::All, termion::cursor::Goto(1, 1));
    println!("{:?} terminal_size", terminal_size());
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
    println!("{}Plain Italic", style::Italic);
}
