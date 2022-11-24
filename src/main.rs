use clap::Parser;
use ctrlc::set_handler;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use xhidecursor::x_hide_cursor;

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long, name = "decode", default_value_t = 0)]
    pub display: i8,
}

fn main() {
    let args = Args::parse();
    let running = Arc::new(AtomicBool::new(true));
    let is_running = running.clone();
    set_handler(move || {
        running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    if !x_hide_cursor(args.display) {
        exit(1);
    }

    while is_running.load(Ordering::SeqCst) {}
}
