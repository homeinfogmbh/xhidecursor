use clap::Parser;
use std::process::exit;
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
    let display = Display::open(Args::parse().display).unwrap_or_else(|err| {
        eprintln!("{err}");
        exit(1);
    });

    display.hide_cursor(display.default_root_window());
    display.sync(true);

    loop {
        sleep(Duration::MAX);
    }
}
