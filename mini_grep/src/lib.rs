use std::{env::{self, Args}, error::Error, fs::read_to_string};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &mut Args) -> Result<Config, &str> {
        args.next(); // skip program name
        let config: Config = match (args.next(), args.next()) {
            (Some(query), Some(filename)) => {
                let case_sensitive = env::var("CASE_SENSITIVE").map(|val| val.to_lowercase() == "true").unwrap_or(false);
                // println!("CASE_SENSITIVE={:?}", case_sensitive);
                Config { query, filename, case_sensitive }
            },
            _ => return Err("Not enough arguments"),
        };
        Ok(config)
    }
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    // vec![]
    let mut ans = Vec::new();
    for line in contents.lines() {
        match case_sensitive {
            true => if line.contains(query) {
                ans.push(line);
            },
            false => if line.to_lowercase().contains(&query.to_lowercase()) {
                ans.push(line);
            },
        }
    }
    ans
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(config.filename)?;
    // println!("With text:\n{}", contents);

    for line in search(&config.query, &contents, config.case_sensitive) {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search(query, contents, false)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }
}
