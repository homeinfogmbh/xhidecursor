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
        Ok(display) => {
            display.hide_cursor(display.default_root_window());
            display.sync(true);

            loop {
                sleep(Duration::MAX);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}
