use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(&config) {
        eprintln!("Could not open file: {}. Error: {}", &config.file_path, err);
        process::exit(1)
    }
}
