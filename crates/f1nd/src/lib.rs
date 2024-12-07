use std::{error, fmt::Debug, fs};

#[derive(Debug)]
pub struct Config {
    path: String,
    keyword: String,
}

impl Config {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // The first arg is the program name. we don't need it.
        args.next();

        Ok(Config {
            path: args.next().expect("No path provided"),
            keyword: args.next().expect("No keyword provided"),
        })
    }
}

pub fn f1nd(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.path)?;

    for line in search(&config.keyword, &contents) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(keyword: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&keyword.to_lowercase()))
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let keyword = "Rust";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";

        assert_eq!(vec!["Rust:"], search(keyword, contents));
    }

    #[test]
    fn case_insensitive() {
        let keyword = "rust";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";

        assert_eq!(vec!["Rust:"], search(keyword, contents))
    }
}
