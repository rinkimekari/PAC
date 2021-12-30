mod draw;
pub mod press;

use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::MouseTerminal;
// use ansi_term::Color;
use std::io::{self, stdout, Stdout, Write};
use std::process;
use std::sync::mpsc::Receiver;


// TODO: make better error system with enums


const LOGO_ASCII: &'static str =
"    _/_/_/      _/_/      _/_/_/\r
   _/    _/  _/    _/  _/\r
  _/_/_/    _/_/_/_/  _/\r
 _/        _/    _/  _/\r
_/        _/    _/    _/_/_/\r\
";

pub enum Command {
    Quit,
}

pub struct Tui {
    width: u16,
    height: u16,
    _stdout: MouseTerminal<RawTerminal<Stdout>>,
    buff: Vec<Vec<char>>,
    event_listener: Receiver<Command>,
}

impl Tui {
    pub fn new_or(
        mut width: u16,
        mut height: u16,
        event_listener: Receiver<Command>) -> Self
    {
        if let Ok((w, h)) = termion::terminal_size() {
            width = w;     // destructuring assignments are unstable
            height = h;
        }

        let stdout = stdout().into_raw_mode().unwrap();

        let _stdout = MouseTerminal::from(stdout);

        let mut buff = Vec::with_capacity(height.into());

        for _ in 0..buff.capacity() {
            let mut n = Vec::with_capacity(width.into());
            for _ in 0..n.capacity() {
                n.push(' ');
            }

            buff.push(n);
        }

        Self {
            width,
            height,
            _stdout,
            buff,
            event_listener,
        }
    }

    fn init_screen(&mut self) {
        self.create_layout();
        draw::flush_buff(&self.buff);
    }

    fn create_layout(&mut self) {
        let split_width = if self.width >> 2 > 15 {
            (self.width as usize) >> 2
        } else {
            0
        };

        let split_height = if self.height - 5 > 15 {
            self.height as usize - 5
        } else {
            0
        };

        draw::rectangle(&mut self.buff,
                        0,
                        0,
                        self.width as usize,
                        self.height as usize);
        draw::rectangle(&mut self.buff,
                        1,
                        1,
                        split_width,
                        split_height);
        draw::rectangle(&mut self.buff,
                        1,
                        self.height as usize - 4,
                        self.width as usize - 2,
                        3);
        let chat_box_x = if split_width == 0 || split_height == 0 {
            1
        } else {
            split_width + 2
        };

        draw::rectangle(&mut self.buff,
                        chat_box_x,
                        1,
                        self.width as usize - chat_box_x - 1,
                        self.height as usize - 5);
    }

    pub fn run(&mut self) {
        self.init_screen();

        loop {
            if let Ok(s) = self.event_listener.try_recv() {
                match s {
                    Command::Quit => Self::quit(),
                }
            }
        }
    }

    fn quit() {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        io::stdout().flush().unwrap();
        process::exit(0);
    }
}


///////////////////////////////////////////////////////////////////////////////

// old code just in case - not used or needed

//pub fn start(tui: Rc<Tui>, key_rx: Receiver<Event>, comm_rx: Receiver<String>) {
//    draw::hide_cursor().unwrap();
//    draw::clear_screen().unwrap();
//
//    let rect = draw::Rectangle::new(
//        Rc::clone(&tui),
//        1, // remember: ansi coords start at 1
//        1,
//        tui.width,
//        tui.height,
//        Color::Blue
//    );
//
//    rect.draw();
//
//    // TODO: check if terminal size changed since last iteration,
//    //       then change tui accordingly.
//    loop {
//        if let Ok(s) = key_rx.try_recv() {
//            press::handle_event(s);
//        }
//        if let Ok(s) = comm_rx.try_recv() {
//            draw::text(Rc::clone(&tui), 5, 5, s, Color::Red);
//        }
//    }
//}
