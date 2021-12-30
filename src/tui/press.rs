use termion::event::{Key, Event, MouseEvent};
use termion::input::TermRead;
use std::sync::mpsc::Sender;
use std::io::stdin;
use super::Command;

pub struct KeypressHandler {
    event_sender: Sender<Command>,
}

impl KeypressHandler {
    pub fn new(event_sender: Sender<Command>) -> Self {
        Self {
            event_sender,
        }
    }

    pub fn listen(&self) {
        let stdin = stdin();
        for i in stdin.events() {
            let event = i.unwrap();
            self.handle_event(event);
        }
    }

    fn handle_event(&self, event: Event) {
        match event {
            Event::Key(k) => self.process_keypress(k),
            Event::Mouse(e) => self.process_mouse(e),
            Event::Unsupported(_) => {}, // TODO,
        }
    }

    fn process_keypress(&self, key: Key) {
        match key {
            Key::Ctrl('q') => self.event_sender.send(Command::Quit).unwrap(),
            _ => {}
        }
    }

    fn process_mouse(&self, event: MouseEvent) {
        match event {
            MouseEvent::Press(_button, _x, _y) => {}
            MouseEvent::Release(_x, _y) => {}
            MouseEvent::Hold(_x, _y) => {}
        }
    }
}
