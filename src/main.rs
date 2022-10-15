use std::{env, process};
use minigrep;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let config = minigrep::Config::build(&argv).unwrap_or_else(|err| {
        println!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Failed to read file contents: {}", e);
        process::exit(1)
    }
}
