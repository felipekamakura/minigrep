use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = read_to_string(file_path).expect("Should have read the file successfully!");

    println!("With text:\n{contents}");
}
