use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_args(&args);
  
    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = read_to_string(file_path).expect("Should have read the file successfully!");

    println!("With text:\n{contents}");
}

fn parse_args(args: &Vec<String>) -> (&String, &String) {

    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
