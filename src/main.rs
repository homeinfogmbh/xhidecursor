use clap::Parser;
use ctrlc::set_handler;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use xhidecursor::Display;

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long, name = "display", default_value = None)]
    pub display: Option<String>,
}

fn main() {
    let args = Args::parse();
    let running = Arc::new(AtomicBool::new(true));
    let is_running = running.clone();
    set_handler(move || {
        running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    match Display::open(args.display) {
        Some(mut display) => {
            let root = display.default_root_window();
            display.hide_cursor(root);
            display.sync(true);
        }
        None => {
            eprintln!("Cannot open display");
            exit(1);
        }
    }

    while is_running.load(Ordering::SeqCst) {
        sleep(Duration::from_secs(1));
    }
}
