// use ansi_term::Color;
use termion::{clear, cursor};
use std::io::{self, Write};
use super::Tui;

// TODO: implement colors

pub trait Draw {
    fn rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize);
    fn frame(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        frame_text: String);
    fn flush_buff(&self);
    fn buff_to_string(&self) -> String;
    fn print_input_char(&mut self, c: char);
    fn print_char(&mut self, c: char, x: usize, y: usize);
    fn write_partial_line(&mut self, x: usize, m_x: usize, y: usize, c: char);
    fn write_text(&mut self, text: String);
}

impl Draw for Tui {
    // TODO: use write_partial_line function in this function
    fn rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize)
    {
        if x + width > self.buff[0].len() + 1 ||
           y + height > self.buff.len() + 1
        {
            panic!("rectangle was too girthy");
        }

        if width <= 1 || height <= 1 {
            return;
        }

        // corners
        let max_w = x + width - 1;
        let max_h = y + height - 1;
        self.buff[y][x]         = '╭';
        self.buff[y][max_w]     = '╮';
        self.buff[max_h][x]     = '╰';
        self.buff[max_h][max_w] = '╯';

        for i in 1..width - 1 {
            self.buff[y][x + i] = '─';
            self.buff[y + height - 1][x + i] = '─';
        }

        for i in 1..height - 1 {
            self.buff[y + i][x] = '│';
            self.buff[y + i][x + width - 1] = '│';
        }
    }

    fn frame(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        frame_text: String)
    {
        let frame_text = format!("{}{}{}", '┤', frame_text, '├');
        self.rectangle(x, y, width, height);
        self.cursor_index = (x + 3, y);
        self.write_text(frame_text);
    }

    fn flush_buff(&self) {
        print!(
            "{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            self.buff_to_string()
        );
        io::stdout().flush().unwrap();
    }

    fn buff_to_string(&self) -> String {
        let mut ret = String::with_capacity(
            self.buff.len() * self.buff[0].len()
        );

        for i in &self.buff {
            for j in i {
                ret.push(*j);
            }
        }
        ret
    }

    fn print_input_char(&mut self, c: char) {
        let (x, y) = self.input_index;

        if x == self.width - 2 { return; }

        self.print_char(c, x, y);
        self.input_index = (x + 1, y);

        self.input_buff.push(c);
    }

    fn print_char(&mut self, c: char, x: usize, y: usize) {
        self.cursor_index = (x, y);
        self.buff[y][x] = c;
        self.cursor_index = (x + 1, y);
        self.flush_buff();
    }

    fn write_partial_line(&mut self, x: usize, m_x: usize, y: usize, c: char) {
        for i in x..m_x {
            self.buff[y][i] = c;
        }
    }

    // DO NOT pass newline or other special characters to this method
    fn write_text(&mut self, text: String) {
        for (i, c) in text.chars().enumerate() {
            self.buff[self.cursor_index.1][self.cursor_index.0 + i] = c;
        }
    }
}
