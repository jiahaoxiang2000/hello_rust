//! # hello_rust
//!
//! `hello_rust` is a learn project. follower the book [The Rust Programming Language](https://doc.rust-lang.org/book/).
//!

pub use self::tools::run;

use std::{env, error::Error, fs};
pub mod tools {
    use super::*;
    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;

        let results = if config.ignore_case {
            println!("Ignoring case");
            search_case_insensitive(&config.query, &contents)
        } else {
            println!("Case sensitive");
            search(&config.query, &contents)
        };

        for line in results {
            println!("{line}");
        }

        Ok(())
    }
}

/// Config struct
///
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Create a new Config instance
    ///
    /// # Examples
    /// ```
    /// use hello_rust_xjh::Config;
    ///
    /// let args = vec!["".to_string(), "duct".to_string(), "poem.txt".to_string()];
    /// let config = Config::build(&args).unwrap();
    /// assert_eq!(config.query, "duct");
    /// assert_eq!(config.file_path, "poem.txt");
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if.
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = match env::var("IGNORE_CASE").unwrap_or("1".to_string()).as_str() {
            "1" => true,
            _ => false,
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
