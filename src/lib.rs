//! This crate is the heart of minigrep

use std::{error, fs};

use text::utils::{search, search_case_insensitive};

pub mod text;

pub fn run(config: &Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    match results[..] {
        [] => eprintln!("No matches!"),
        _ => results.iter().for_each(|line| println!("{line}")),
    }

    Ok(())
}

/// # Config Model
/// ## Motivation
/// This is a model value that carries all the configuration values coming from input **args** and also environment variables.
///
/// Support variables:
/// - `IGNORE_CASE`: when set, runs the search ignoring case
///
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
