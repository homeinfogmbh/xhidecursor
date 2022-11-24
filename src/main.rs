use clap::Parser;
use ctrlc::set_handler;
use std::ffi::CString;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use xhidecursor::x_hide_cursor;

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long, name = "display", default_value_t = String::from(":0"))]
    pub display: String,
}

fn main() {
    let args = Args::parse();
    let running = Arc::new(AtomicBool::new(true));
    let is_running = running.clone();
    set_handler(move || {
        running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    match CString::new(args.display) {
        Ok(display) => {
            if !x_hide_cursor(&display) {
                exit(1);
            }
        }
        Err(err) => {
            eprintln!("Cannot parse CString: {}", err);
        }
    }

    while is_running.load(Ordering::SeqCst) {}
}
