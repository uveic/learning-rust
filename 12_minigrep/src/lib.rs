use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    // Before:
    // pub fn new(args: &[String]) -> Result<Config, &str> {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_insensitive = match env::var("CASE_INSENSITIVE") {
            Ok(r) => r.len() > 0,
            Err(_) => args.filter(|a| a.contains("--case-insensitive")).count() > 0
        };

        Ok(Self {
            query,
            filename,
            case_sensitive: !case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = match config.case_sensitive {
        true => search_case_sensitive(&config.query, &contents),
        false => search_case_insensitive(&config.query, &contents),
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

    // Before: not using Iterator capabilities
    //
    // let mut results = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    //
    // results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn run() {
        let config = Config::new(
            &[
                String::from("path/to/self"),
                String::from("needle"),
                String::from("filename.txt"),
            ]
        ).unwrap();

        let _res = super::run(config).unwrap();
    }

    #[test]
    fn create_config() {
        let config = Config::new(
            &[
                String::from("path/to/self"),
                String::from("needle"),
                String::from("filename.txt"),
            ]
        ).unwrap();

        assert_eq!(config.query, String::from("needle"));
        assert_eq!(config.filename, String::from("filename.txt"));
    }

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            super::search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            super::search_case_insensitive(query, contents),
        );
    }
}
