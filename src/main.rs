mod cli;
mod tui;
mod comm;

use tui::Tui;
use std::thread;
use std::sync::mpsc;
// use std::rc::Rc;
use tui::press::KeypressHandler;

fn main() {
    let args = cli::init();
    if let Some(t) = args.value_of("TEST") {
        cli::handle_test(t);
    }

    let (event_sender, event_listener) = mpsc::channel();
    // let (comm_tx, comm_rx) = mpsc::channel();

    // let comm_thread = thread::spawn(move || {
    //     comm::connect(comm_tx);
    // });

    // NOTE: think about whether or not to Rc the sender

    let keypress_thread = thread::spawn(move || {
        let key_handler = KeypressHandler::new(event_sender);
        key_handler.listen();
    });

    let tui_thread = thread::spawn(move || {
        let mut tui = Tui::new_or(100, 80, event_listener);
        tui.run();
    });

    // comm_thread.join().unwrap();
    keypress_thread.join().unwrap();
    tui_thread.join().unwrap();
}
