use std::{env, fs, process};

pub fn parse_config(args: &[String]) -> Config {
    let query = &args[1].as_str();
    let file_path = &args[2].as_str();
    Config { query, file_path }
}

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

// TODO: DELETE: replaced by build; not needed
impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Config<'a> {
        if args.len() < 3 {
            panic!("not enough arguments!")
        }
        let query = args[1].as_str();
        let file_path = args[2].as_str();
        Config { query, file_path }
    }
}

// TODO: check if it is OK to use &str here
impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].as_str();
        let file_path = args[2].as_str();
        Ok(Config { query, file_path })
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments!: {err}");
        process::exit(1);
    });
    let contents = fs::read_to_string(config.file_path)
        .expect("File Read Error: either file does not exist or incorrect path!");
    println!("With {}:\n{}", config.query, contents)
}
