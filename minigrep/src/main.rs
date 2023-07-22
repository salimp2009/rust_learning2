use std::{env, fs};
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for '{query}' ");
    println!("In file '{file_path}' ");

    let contents = fs::read_to_string(file_path)
        .expect("File Read Error: either file does not exist or incorrect path!");
    println!("With {query}:\n{contents}")
}
