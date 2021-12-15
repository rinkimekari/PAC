use termion::event::{Key, Event, MouseEvent};

pub fn handle_event(event: &Event) {
    match event {
        Event::Key(k) => process_keypress(*k),
        Event::Mouse(e) => process_mouse(*e),
        Event::Unsupported(_) => {}, // TODO,
    }
}

fn process_keypress(key: Key) {
    match key {
        Key::Char('q') => super::Tui::quit(),
        _ => {}
    }
}

fn process_mouse(event: MouseEvent) {
    match event {
        MouseEvent::Press(_button, _x, _y) => {}
        MouseEvent::Release(_x, _y) => {}
        MouseEvent::Hold(_x, _y) => {}
    }
}
