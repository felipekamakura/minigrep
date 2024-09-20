use std::{env, process};
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(&config) {
        eprintln!("Could not open file: {}. Error: {}", &config.file_path, err);
        process::exit(1)
    }    
}
