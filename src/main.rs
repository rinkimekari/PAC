mod cli;
mod tui;
mod comm;

use tui::Tui;
use std::thread;
use std::sync::mpsc;

fn main() {
    let args = cli::init();
    if let Some(t) = args.value_of("TEST") {
        cli::handle_test(t);
    }

    let mut tui = Tui::new_or(100, 80);

    let (key_tx, key_rx) = mpsc::channel();
    let (comm_tx, comm_rx) = mpsc::channel();

    let comm_thread = thread::spawn(move || {
        comm::connect(comm_tx);
    });

    let keypress_thread = thread::spawn(move || {
        tui::press::start(key_tx);
    });

    tui.start(key_rx, comm_rx);

    comm_thread.join().unwrap();
    keypress_thread.join().unwrap();
}
