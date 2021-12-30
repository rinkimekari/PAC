// use ansi_term::Color;
use termion::{clear, cursor};
use std::io::{self, Write};

// TODO: implement colors

pub fn rectangle(
    buff: &mut Vec<Vec<char>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize)
{
    if x + width > buff[0].len() + 1 ||
       y + height > buff.len() + 1
    {
        panic!("rectangle was too girthy");
    }

    if width <= 1 || height <= 1 {
        return;
    }

    // corners
    let max_w = x + width - 1;
    let max_h = y + height - 1;
    buff[y][x]         = '╭';
    buff[y][max_w]     = '╮';
    buff[max_h][x]     = '╰';
    buff[max_h][max_w] = '╯';

    for i in 1..width - 1 {
        buff[y][x + i] = '─';
        buff[y + height - 1][x + i] = '─';
    }

    for i in 1..height - 1 {
        buff[y + i][x] = '│';
        buff[y + i][x + width - 1] = '│';
    }
}

pub fn flush_buff(buff: &Vec<Vec<char>>) {
    print!("{}{}{}", clear::All, cursor::Goto(1, 1), buff_to_string(buff));
    io::stdout().flush().unwrap();
}

fn buff_to_string(buff: &Vec<Vec<char>>) -> String {
    let mut ret = String::with_capacity(buff.len() * buff[0].len());
    for i in buff {
        for j in i {
            ret.push(*j);
        }
    }
    ret
}


///////////////////////////////////////////////////////////////////////////////

// old code just in case - not used or needed

//pub struct Rectangle {
//    tui: Rc<Tui>, // 'static lasts entire program
//    x: u16,
//    y: u16,
//    width: u16,
//    height: u16,
//    color: Color,
//}
//
//impl Rectangle {
//    pub fn new(
//        tui: Rc<Tui>,
//        x: u16,
//        y: u16,
//        width: u16,
//        height: u16,
//        color: Color
//    ) -> Self {
//        if x + width > tui.width + 1 || y + height > tui.height + 1 {
//            show_cursor().unwrap();
//            panic!("rectangle was too girthy");
//        }
//
//        if width <= 1 || height <= 1 {
//            show_cursor().unwrap();
//            panic!("Rectangle was too small");
//        }
//
//        if x == 0 || y == 0 {
//            show_cursor().unwrap();
//            panic!("ANSI starts at 1");
//        }
//
//        Self {
//            tui: Rc::clone(&tui),
//            x,
//            y,
//            width,
//            height,
//            color,
//        }
//    }
//
//    pub fn draw(&self) {
//        // top left corner
//        move_cursor(self.x, self.y).unwrap();
//        print!("{}", self.color.paint("╭"));
//
//        // top right corner
//        move_cursor(self.x + self.width - 1, self.y).unwrap();
//        print!("{}", self.color.paint("╮"));
//
//        // bottom left corner
//        move_cursor(self.x, self.y + self.height - 1).unwrap();
//        print!("{}", self.color.paint("╰"));
//
//        // bottom right corner
//        move_cursor(self.x + self.width - 1, self.y + self.height - 1).unwrap();
//        print!("{}", self.color.paint("╯"));
//
//        // top and bottom
//        for i in 1..self.width - 1 {
//            move_cursor(self.x + i, self.y).unwrap();
//            print!("{}", self.color.paint("─"));
//
//            move_cursor(self.x + i, self.y + self.height - 1).unwrap();
//            print!("{}", self.color.paint("─"));
//        }
//
//        // left and right
//        for i in 1..self.height - 1 {
//            move_cursor(self.x, self.y + i).unwrap();
//            print!("{}", self.color.paint("│"));
//
//            move_cursor(self.x + self.width - 1, self.y + i).unwrap();
//            print!("{}", self.color.paint("│"));
//        }
//
//        io::stdout().flush().unwrap();
//    }
//
//    pub fn change_color(&mut self, color: Color) {
//        self.color = color;
//        self.draw();
//    }
//}

// pub fn clear_screen() -> Result<(), io::Error> {
//     print!("{}{}",
//            clear::All,
//            cursor::Goto(1, 1) // ansi starts at 1
//     );
//     io::stdout().flush()
// }
//
// pub fn hide_cursor() -> Result<(), io::Error> {
//     print!("{}", cursor::Hide);
//     io::stdout().flush()
// }
//
// pub fn show_cursor() -> Result<(), io::Error> {
//     print!("{}", cursor::Show);
//     io::stdout().flush()
// }
//
// pub fn move_cursor(x: u16, y: u16) -> Result<(), io::Error> {
//     print!("{}", cursor::Goto(x, y));
//     io::stdout().flush()
// }

// pub fn text(
//     tui: Rc<Tui>,
//     x: u16,
//     y: u16,
//     text: String,
//     color: Color
// ) {
//     if x <= 1 || y <= 1 || x >= tui.width - 1 || y >= tui.height - 1 {
//         show_cursor().unwrap();
//         panic!("yeah out of bounds buddy");
//     }
//
//     move_cursor(x, y).unwrap();
//
//     // TODO: make line wrap work cuz im stupid
//
//     // let mut bound = 0;
//     // let mut newline_num = 2;
//
//     // for (i, c) in text.chars().enumerate() {
//     //     let i = i as u16;
//     //     if x + i >= tui.width && bound == 0 {
//     //         bound = x + i;
//     //     }
//     //     if x + i - bound * (newline_num - 1)  >= tui.width {
//     //         move_cursor(x, y + newline_num).unwrap();
//     //         newline_num += 1;
//     //     }
//     //     print!("{}", color.paint(String::from(c)));
//     // }
//
//     print!("{}", color.paint(text));
//
//     io::stdout().flush().unwrap();
// }
