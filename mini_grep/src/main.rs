use std::env;
use mini_grep::Config;

fn main() {
    let mut args = env::args();
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Args parsing Error: {}", err);
        std::process::exit(1);
    });

    println!(
        "Searching for \"{}\" in file \"{}\"",
        config.query, config.filename
    );
    if let Err(e) = mini_grep::run(config) {
        eprintln!("Run Error: {}", e);
        std::process::exit(1);
    }
}