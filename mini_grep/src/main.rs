use std::{
    env::{self, Args},
    error::Error,
    fs::read_to_string,
};

fn main() {
    // println!("Hello, world!");
    let mut args = env::args();
    // let config = match Config::new(&mut args) {
    //     Ok(config) => config,
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         return;
    //     }
    // };
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Args parsing Error: {}", err);
        std::process::exit(1);
    });

    // let contents = match read_to_string(config.filename) {
    //     Ok(contents) => contents,
    //     Err(e) => match e.kind() {
    //         std::io::ErrorKind::NotFound => {
    //             println!("File not found");
    //             return;
    //         }
    //         _ => {
    //             println!("Error: {}", e);
    //             return;
    //         }
    //     },
    // };

    println!(
        "Searching for \"{}\" in file \"{}\"",
        config.query, config.filename
    );
    if let Err(e) = run(config) {
        eprintln!("Run Error: {}", e);
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &mut Args) -> Result<Config, &str> {
        // if args.len() < 3 {
        //     println!("Not enough arguments");
        //     return Err("Not enough arguments".to_string());
        // }
        // let query = &args[1];
        // let filename = &args[2];
        // return Ok((query.clone(), filename.clone()));
        args.next(); // skip program name
        let config: Config = match (args.next(), args.next()) {
            (Some(query), Some(filename)) => Config { query, filename },
            _ => return Err("Not enough arguments"),
        };
        Ok(config)
    }
}
