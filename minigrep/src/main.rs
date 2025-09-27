use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query} in {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    println!("With contents:\n{contents}");
}
