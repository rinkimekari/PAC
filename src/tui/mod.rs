mod draw;
pub mod press;

use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::MouseTerminal;
// use ansi_term::Color;
use std::io::{self, stdout, Stdout, Write};
use std::process;
use std::sync::mpsc::{Receiver, Sender};
use draw::Draw;


// TODO: make welcome screen
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
    PrintInput(char),
}

pub struct Tui {
    width: usize,
    height: usize,
    _stdout: MouseTerminal<RawTerminal<Stdout>>,
    buff: Vec<Vec<char>>,
    event_listener: Receiver<Command>,
    comm_message_sender: Sender<String>,
    cursor_index: (usize, usize),
    input_index: (usize, usize),
    input_buff: String,
}

impl Tui {
    pub fn new(
        mut width: usize,
        mut height: usize,
        event_listener: Receiver<Command>,
        comm_message_sender: Sender<String>) -> Self
    {
        if let Ok((w, h)) = termion::terminal_size() {
            width = w as usize;     // destructuring assignments are unstable
            height = h as usize;
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

        let cursor_index = (0, 0);
        let input_index = (0, 0);
        let input_buff = String::new();

        Self {
            width,
            height,
            _stdout,
            buff,
            event_listener,
            comm_message_sender,
            cursor_index,
            input_index,
            input_buff,
        }
    }

    fn init_screen(&mut self) {
        self.create_layout();
        self.flush_buff();
    }

    fn create_layout(&mut self) {
        let split_width = if self.width >> 2 > 15 {
            (self.width ) >> 2
        } else {
            0
        };

        let split_height = if self.height - 5 > 15 {
            self.height - 5
        } else {
            0
        };

        self.rectangle(0, 0, self.width, self.height);
        self.rectangle(1, 1, split_width, split_height);
        self.rectangle(1, self.height - 4, self.width - 2, 3);

        let chat_box_x = if split_width == 0 || split_height == 0 {
            1
        } else {
            split_width + 2
        };

        self.rectangle(chat_box_x,
                       1,
                       self.width - chat_box_x - 1,
                       self.height - 5);

        self.input_index = (2, self.height - 3);
    }

    pub fn run(&mut self) {
        self.init_screen();

        loop {
            if let Ok(s) = self.event_listener.recv() {
                match s {
                    Command::Quit => Self::quit(),
                    Command::PrintInput(c) => match c {
                        '\t' | '\r' => {}
                        '\n' => self.send_message(),
                        _ => self.print_input_char(c),
                    }
                }
            }
        }
    }

    fn send_message(&mut self) {
        if self.input_buff.is_empty() { return; }

        self.comm_message_sender.send(
            String::from(self.input_buff.as_str())
        ).unwrap();

        self.reset_input();
    }

    fn reset_input(&mut self) {
        self.input_buff.clear();
        self.move_cursor_index(2, self.height - 3);
        self.move_input_index(2, self.height - 3);
        self.clear_partial_line(2, self.width - 2, self.height - 3);
        self.flush_buff();
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
