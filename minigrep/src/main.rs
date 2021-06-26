use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("problem parsing argument with error: {}", error);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(&config) {
        eprintln!("application error: {}", e);
        process::exit(1)
    }
}
