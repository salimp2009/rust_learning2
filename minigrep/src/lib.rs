use std::{env, error::Error, fs};

pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].as_str();
    let file_path = args[2].as_str();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    Config {
        query,
        file_path,
        ignore_case,
    }
}

// #[derive(Debug, Clone, Copy)]
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}

// TODO: check if it is OK to use &str here
impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let query = args[1].as_str();
        let file_path = args[2].as_str();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    if config.ignore_case {
        search_case_insensitive(config.query, &contents)
            .iter()
            .for_each(|line| println!("{line}"));
    } else {
        search(config.query, &contents)
            .iter()
            .for_each(|line| println!("{line}"));
    }

    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| line.contains(query))
        .collect::<Vec<&str>>()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|&line| line.to_lowercase().contains(&query))
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensivite() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let test_text = vec!["safe, fast, productive."];
        assert_eq!(test_text, search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";
        let test_text = vec!["Rust:", "Trust me."];
        assert_eq!(test_text, search_case_insensitive(query, contents));
    }

    // #[test]
    // fn path_test(){
    //     let path = env::
    //
    // }
}
