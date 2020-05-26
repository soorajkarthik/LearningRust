use std::env;
use std::process;
use std::time::SystemTime;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("\t Searching for \"{}\" in file: {}\n\n\n", config.query, config.filename);

    let start = std::time::SystemTime::now();

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    let end = std::time::SystemTime::now();
    let time = end.duration_since(start).unwrap().as_millis();
    println!("{}", time);
}