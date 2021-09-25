use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Self { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn one_result() {
        let query = "grep";
        let contents = "\
minigrep
A tiny version of grep built in Rust.

This tool was built only for education purposes using The Rust Book.";
        let expected = vec!["minigrep", "A tiny version of grep built in Rust."];

        assert_eq!(expected, search(query, contents));
    }
}
