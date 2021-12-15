mod cli;
mod tui;

use tui::Tui;

fn main() {
    let args = cli::init();
    if let Some(t) = args.value_of("TEST") {
        cli::handle_test(t);
    }

    Tui::new_or(100, 80).start();
}
