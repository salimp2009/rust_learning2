use std::{env, fs};

pub fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let file_path = &args[2];
    Config { query, file_path }
}

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.file_path)
        .expect("File Read Error: either file does not exist or incorrect path!");
    println!("With {}:\n{}", config.query, contents)
}
