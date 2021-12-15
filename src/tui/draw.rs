use crate::Tui;
use ansi_term::Color;
use termion::{clear, cursor};
use std::io::{self, Write};

pub fn clear_screen() -> Result<(), io::Error> {
    print!("{}{}",
           clear::All,
           cursor::Goto(1, 1) // ansi starts at 1
    );
    io::stdout().flush()
}

pub fn hide_cursor() -> Result<(), io::Error> {
    print!("{}", cursor::Hide);
    io::stdout().flush()
}

pub fn show_cursor() -> Result<(), io::Error> {
    print!("{}", cursor::Show);
    io::stdout().flush()
}

pub fn move_cursor(x: u16, y: u16) -> Result<(), io::Error> {
    print!("{}", cursor::Goto(x, y));
    io::stdout().flush()
}

pub fn rectangle(
    tui: &Tui,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    color: Color
) {
    if x + width > tui.width + 1 || y + height > tui.height + 1 {
        show_cursor().unwrap();
        panic!("Rectangle was too girthy");
    }

    if width <= 1 || height <= 1 {
        show_cursor().unwrap();
        panic!("Rectangle was too small");
    }

    if x == 0 || y == 0 {
        show_cursor().unwrap();
        panic!("ANSI starts at 1");
    }

    // top left corner
    move_cursor(x, y).unwrap();
    print!("{}", color.paint("╭"));

    // top right corner
    move_cursor(x + width - 1, y).unwrap();
    print!("{}", color.paint("╮"));

    // bottom left corner
    move_cursor(x, y + height - 1).unwrap();
    print!("{}", color.paint("╰"));

    // bottom right corner
    move_cursor(x + width - 1, y + height - 1).unwrap();
    print!("{}", color.paint("╯"));

    // top and bottom
    for i in 1..width - 1 {
        move_cursor(x + i, y).unwrap();
        print!("{}", color.paint("─"));

        move_cursor(x + i, y + height - 1).unwrap();
        print!("{}", color.paint("─"));
    }

    // left and right
    for i in 1..height - 1 {
        move_cursor(x, y + i).unwrap();
        print!("{}", color.paint("│"));

        move_cursor(x + width - 1, y + i).unwrap();
        print!("{}", color.paint("│"));
    }

    io::stdout().flush().unwrap();
}
