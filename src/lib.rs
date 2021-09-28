use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("missing argument (query string)"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("missing argument (file name)"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            matching.push(line);
        }
    }

    matching
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matching = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matching.push(line);
        }
    }

    matching
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "grep";
        let contents = "\
minigrep
A tiny version of grep built in Rust.

This tool was built only for education purposes using The Rust Book.

The word \"Grep\" shouldn't match, because the search is case-sensitive.";
        let expected = vec!["minigrep", "A tiny version of grep built in Rust."];

        assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
minigrep
A tiny version of grep built in Rust.

This tool was built only for education purposes using The Rust Book.";
        let expected = vec![
            "A tiny version of grep built in Rust.",
            "This tool was built only for education purposes using The Rust Book.",
        ];

        assert_eq!(expected, search_case_insensitive(query, contents));
    }
}
