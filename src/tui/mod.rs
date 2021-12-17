mod draw;
pub mod press;

use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::MouseTerminal;
use termion::event::Event;
use ansi_term::Color;
use std::io::{stdout, Stdout};
use std::process;
use std::sync::mpsc::Receiver;


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

    pub fn start(&mut self, key_rx: Receiver<Event>, comm_rx: Receiver<String>) {
        draw::hide_cursor().unwrap();
        draw::clear_screen().unwrap();

        draw::rectangle(self,
                        1, // remember: ansi coords start at 1
                        1,
                        self.width,
                        self.height,
                        Color::Blue);

        // TODO: check if terminal size changed since last iteration,
        //       then change tui accordingly.
        loop {
            if let Ok(s) = key_rx.try_recv() {
                press::handle_event(s);
            }
            if let Ok(s) = comm_rx.try_recv() {
                draw::text(self, 5, 5, s, Color::Red);
            }
        }
   }

    fn quit() {
        draw::clear_screen().unwrap();
        draw::show_cursor().unwrap();
        process::exit(0);
    }
}
