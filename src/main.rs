mod cli;
mod tui;
mod comm;

use tui::Tui;
use std::thread;
use std::sync::mpsc;
use tui::press::KeypressHandler;

fn main() {
    let args = cli::init();
    if let Some(t) = args.value_of("TEST") {
        cli::handle_test(t);
    }

    let (event_sender, event_listener) = mpsc::channel();
    let (comm_message_sender, comm_message_receiver) = mpsc::channel();

    let comm_thread = thread::spawn(move || {
        comm::connect(comm_message_receiver);
    });

    let keypress_thread = thread::spawn(move || {
        let key_handler = KeypressHandler::new(event_sender);
        key_handler.listen();
    });

    let tui_thread = thread::spawn(move || {
        let mut tui = Tui::new(100, 80, event_listener, comm_message_sender);
        tui.run();
    });

    comm_thread.join().unwrap();
    keypress_thread.join().unwrap();
    tui_thread.join().unwrap();
}
