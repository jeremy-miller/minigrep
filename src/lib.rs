//! # Minigrep
//!
//! `minigrep` is a tool for searching a particular file for specific text.  It can perform
//! both case-sensitive and case-insensitive searches.

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
    /// Whether or not to execute a case-sensitive search.  This is set via the `CASE_INSENSITIVE`
    /// environment variable.
    pub case_sensitive: bool,
}

impl Config {
    /// Creates a new [`Config`](struct.Config.html).
    ///
    /// # Arguments
    ///
    /// * `args` - Command line arguments for this `minigrep` execution.
    ///
    /// # Return
    ///
    /// `Result` of either a valid [`Config`](struct.Config.html), or an `Err` with a string
    /// literal error message.
    ///
    /// # Errors
    ///
    /// * Returns a string literal `Err` if no [`query`](struct.Config.html#structfield.query)
    ///   argument is passed.
    /// * Returns a string literal `Err` if no [`filename`](struct.Config.html#structfield.filename)
    ///   argument is passed.
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // skip the first argument (the program name)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        // using .is_err() to set `case_sensitive` to `true` if `CASE_INSENSITIVE` is unset
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

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
///
/// * `config` - [`Config`](struct.Config.html) containing the arguments to use for this search
/// execution.
///
/// # Return
///
/// `Result` of either `()` if successful, or an `Error` trait object.
///
/// # Errors
///
/// * Returns errors related to opening [`filename`](struct.Config.html#structfield.filename)
///   (e.g. file doesn't exist, permissions issues, etc).
/// * Returns an error if [`filename`](struct.Config.html#structfield.filename) does not contain
///   valid UTF-8 data.
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

/// Executes a case-sensitive text search of a string.
///
/// # Arguments
///
/// * `query` - [`query`](struct.Config.html#structfield.query) to search for.
/// * `contents` - The full text string to search for [`query`](fn.search.html#arguentsfield.query)
///                instances.
///
/// # Return
///
/// `Vec` of lines from [`contents`](fn.search.html#arguentsfield.contents) which match the
/// [`query`](fn.search.html#arguentsfield.query).
///
/// # Examples
///
/// ```
/// let query = "duct";
/// let contents = "\
///Rust:
///safe, fast, productive.
///Pick three.
///Duct tape.";
///
/// assert_eq!(vec!["safe, fast, productive."], minigrep::search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Executes a case-insensitive text search of a string.  This function is called when the
/// `CASE_INSENSITIVE` environment variable is set when calling [`run`](fn.run.html).
///
/// # Arguments
///
/// * `query` - [`query`](struct.Config.html#structfield.query) to search for.
/// * `contents` - The full text string to search for
///                [`query`](fn.search_case_insensitive.html#arguentsfield.query) instances.
///
/// # Return
///
/// `Vec` of lines from [`contents`](fn.search_case_insensitive.html#arguentsfield.contents) which
/// match the [`query`](fn.search_case_insensitive.html#arguentsfield.query).
///
/// # Examples
///
/// ```
/// let query = "rUsT";
/// let contents = "\
///Rust:
///safe, fast, productive.
///Pick three.
///Trust me.";
///
/// assert_eq!(
///     vec!["Rust:", "Trust me."],
///     minigrep::search_case_insensitive(query, contents)
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
