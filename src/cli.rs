use clap::{ArgMatches, clap_app};

pub fn init<'a>() -> ArgMatches<'a> {
    clap_app!(pac =>
        (version: clap::crate_version!())
        (author: &*format!(         // coerces String to &str
            "Copyright (C) 2021 {}",
            clap::crate_authors!()
        ))
        (about: "(P)ainfully (A)wful (C)ommunicator\n\
                A messaging TUI focused on privacy and security")
        (@arg TEST: -t --test +takes_value "Test command (will change later)")
    ).get_matches()
}

pub fn handle_test(t: &str) {
    println!("{}", t);
}
