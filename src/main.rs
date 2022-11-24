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
    let args = Args::parse();

    match Display::open(args.display) {
        Some(mut display) => {
            let root = display.default_root_window();
            display.hide_cursor(root);
            display.sync(true);
        }
        None => {
            eprintln!("Cannot open display.");
            exit(1);
        }
    }

    loop {
        sleep(Duration::MAX)
    }
}
