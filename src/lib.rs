use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

/// Stores configuration parameters for the `minigrep` execution.
pub struct Config {
    /// Query string to be searched for.
    pub query: String,
    /// File to be searched.
    pub filename: String,
    /// Whether or not to execute a case-sensitive search.
    pub case_sensitive: bool,
}

impl Config {
    /// Creates a new `Config`.
    ///
    /// # Arguments
    /// * `args` - Command line arguments for this `minigrep` execution.
    ///
    /// # Return
    /// `Result` of either a valid [`Config`](struct.Config.html), or an `Err` with a string
    /// literal error message.
    ///
    /// # Errors
    /// * Returns a string literal `Err` if no `query` argument is passed.
    /// * Returns a string literal `Err` if no `filename` argument is passed.
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {  // 'static is the lifetime of string literals (error message)
        args.next(); // skip the first argument (the program name)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();  // using .is_err() to set `case_sensitive` to `true` if `CASE_INSENSITIVE` is unset

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// Executes a case-sensitive or case-insensitive text search of a file based on the
/// configuration parameters.  Prints matching lines to `stdout`.
///
/// # Arguments
/// * `config` - [`Config`](struct.Config.html) containing the arguments to use for this search
/// execution.
///
/// # Return
/// `Result` of either `()` if successful, or an `Error` trait object.
///
/// # Errors
/// * Returns errors related to opening `filename` (e.g. file doesn't exist, permissions issues, etc).
/// * Returns an error if `filename` does not contain valid UTF-8 data.
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod lib_tests {
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
}
