mod draw;
mod press;

use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::{MouseTerminal, TermRead};
use ansi_term::Color;
use std::io::{stdin, stdout, Stdout};
use std::process;


// TODO: make better error system with enums


pub struct Tui {
    width: u16,
    height: u16,
    _stdout: MouseTerminal<RawTerminal<Stdout>>,
}

impl Tui {
    pub fn new_or(mut width: u16, mut height: u16) -> Self {
        if let Ok((w, h)) = termion::terminal_size() {
            width = w;     // destructuring assignments are unstable
            height = h;
        }

        let stdout = stdout().into_raw_mode().unwrap();

        let _stdout = MouseTerminal::from(stdout);

        Self {
            width,
            height,
            _stdout,
        }
    }

    pub fn start(&mut self) {
        draw::hide_cursor().unwrap();
        draw::clear_screen().unwrap();

        draw::rectangle(self,
                        1, // remember: ansi coords start at 1
                        1,
                        self.width,
                        self.height,
                        Color::Blue);

        // NOTE: probably multithread to receive messages and update screen
        // TODO: check if terminal size changed since last iteration,
        //       then change tui accordingly. NOTE: cannot be done until
        //       multithreading is implemented
        loop {
            let stdin = stdin();
            for i in stdin.events() {
                let event = i.unwrap_or_else(|e| {
                    draw::show_cursor().unwrap();
                    panic!("{}", e);
                });

                press::handle_event(&event);
            }
        }
    }

    fn quit() {
        draw::clear_screen().unwrap();
        draw::show_cursor().unwrap();
        process::exit(0);
    }
}
