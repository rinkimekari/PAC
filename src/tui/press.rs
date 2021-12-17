use termion::event::{Key, Event, MouseEvent};
use termion::input::TermRead;
use std::sync::mpsc::Sender;
use std::io::stdin;

pub fn start(key_tx: Sender<Event>) {
    let stdin = stdin();
    for i in stdin.events() {
        let event = i.unwrap_or_else(|e| {
            super::draw::show_cursor().unwrap();
            panic!("{}", e);
        });

        key_tx.send(event).unwrap();
    }
}

pub fn handle_event(event: Event) {
    match event {
        Event::Key(k) => process_keypress(k),
        Event::Mouse(e) => process_mouse(e),
        Event::Unsupported(_) => {}, // TODO,
    }
}

fn process_keypress(key: Key) {
    match key {
        Key::Char('q') => super::Tui::quit(),
        _ => { return; }
    }
}

fn process_mouse(event: MouseEvent) {
    match event {
        MouseEvent::Press(_button, _x, _y) => {}
        MouseEvent::Release(_x, _y) => {}
        MouseEvent::Hold(_x, _y) => {}
    }
}
