mod draw;
pub mod press;

use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::MouseTerminal;
// use ansi_term::Color;
use std::io::{self, stdout, Stdout, Write};
use std::process;
use std::sync::mpsc::{Receiver, Sender};
use draw::Draw;


// TODO: change to using self.cursor_index index of passing x and y parameters
// TODO: make line_update and char_update functions
// TODO: make better error system with enums


const LOGO_ASCII: &'static str =
"    _/_/_/      _/_/      _/_/_/
   _/    _/  _/    _/  _/
  _/_/_/    _/_/_/_/  _/
 _/        _/    _/  _/
_/        _/    _/    _/_/_/";

pub enum Command {
    Quit,
    PrintInput(char),
    DeleteInputChar,
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
    split_width: usize,
    split_height: usize,
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

        let split_width = if width >> 2 > 15 {
            (width ) >> 2
        } else {
            0
        };

        let split_height = height - 5;

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
            split_width,
            split_height,
        }
    }

    fn init_screen(&mut self) {
        print!("{}", termion::cursor::Hide);
        io::stdout().flush().unwrap();
        self.create_layout();
        self.logo();
        self.flush_buff();
    }

    fn logo(&mut self) {
        let logo: Vec<&str> = LOGO_ASCII.split('\n').collect();
        self.cursor_index = (self.split_width + 5, 3);
        self.write_text(String::from("Welcome to"));
        self.cursor_index.1 += 2;
        for i in 0..logo.len() {
            self.write_text(String::from(logo[i]));
            self.cursor_index.1 += 1;
        }
    }

    fn create_layout(&mut self) {
        self.rectangle(0, 0, self.width, self.height);
        self.frame(1, 1, self.split_width, self.split_height, String::from("Friends"));
        self.frame(1, self.height - 4, self.width - 2, 3, String::from("Input"));

        let chat_box_x = if self.split_width == 0 {
            1
        } else {
            self.split_width + 2
        };

        self.frame(chat_box_x,
                       1,
                       self.width - chat_box_x - 1,
                       self.split_height,
                       String::from("Chat"));

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
                    Command::DeleteInputChar => self.del_in_char(),
                }
            }
        }
    }

    fn del_in_char(&mut self) {
        if self.input_index.0 == 2 { return; }
        self.input_buff.pop();
        self.buff[self.input_index.1][self.input_index.0 - 1] = ' ';
        self.input_index = (self.input_index.0 - 1, self.input_index.1);
        self.flush_buff();
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
        self.input_index = (2, self.height - 3);
        self.write_partial_line(2, self.width - 2, self.height - 3, ' ');
        self.flush_buff();
    }

    fn quit() {
        print!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), termion::cursor::Show);
        io::stdout().flush().unwrap();
        process::exit(0);
    }
}
