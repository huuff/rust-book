use std::process;
use minigrep::config::Config;
use clap::Parser;

fn main() {
    let config = Config::parse();

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

