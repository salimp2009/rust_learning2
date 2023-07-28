use std::{error::Error, fs};

pub fn parse_config(args: &[String]) -> Config {
    let query = &args[1].as_str();
    let file_path = &args[2].as_str();
    Config { query, file_path }
}

// #[derive(Debug, Clone, Copy)]
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With {}:\n{}", config.query, contents);
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| line.contains(query))
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn failing_test1() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let test_text = vec!["safe, fast, productive."];
        assert_eq!(test_text, search(query, contents));
    }
}
