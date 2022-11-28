use clap::Parser;
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};
use std::process::exit;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use x11oo::Display;

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long, name = "display", default_value = None)]
    pub display: Option<String>,
}

fn main() {
    let args = Args::parse();

    match Display::open(args.display) {
        Ok(display) => {
            display.hide_cursor(display.default_root_window());
            display.sync(true);
            wait_for_term_signal();
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

fn wait_for_term_signal() {
    match Signals::new([SIGINT, SIGTERM]) {
        Ok(mut signals) => {
            thread::spawn(move || {
                for _ in signals.forever() {
                    exit(0);
                }
            });
        }
        Err(_) => exit(2),
    }

    loop {
        sleep(Duration::MAX);
    }
}
