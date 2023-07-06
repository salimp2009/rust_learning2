use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::net::IpAddr;

pub fn recoverables_err1() {
    let greeting_file_result = std::fs::File::open("./src/hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./src/hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Prpblem creating file {:#?}", err),
            },
            other_error => panic!("problem openning the file {:#?}", other_error),
        },
    };
}

pub fn recoverables_err2() {
    let _greeting_file = std::fs::File::open("./src/hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("./src/hello2.txt").unwrap_or_else(|err| {
                panic!("Cannot create file: {:#?}", err);
            })
        } else {
            panic!("Error openning file : {:#?}", error);
        }
    });
}

pub fn read_user_name_frm_file() -> Result<String, io::Error> {
    let user_name_file_result = File::open("./src/hello.txt");

    let mut user_name_file = match user_name_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match user_name_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn read_user_name_frm_file_short() -> Result<String, io::Error> {
    let mut user_name_file = File::open("./src/hello.txt")?;
    let mut username = String::new();

    user_name_file.read_to_string(&mut username)?;
    Ok(username)
}

pub fn last_char_of_firstline(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

pub fn error_info() {
    let home = "127.0.0.1"
        .parse::<IpAddr>()
        .expect("Hardcoded IP address must be valid");
    println!("IpAddress: {home}");
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        // assert!(
        //     (1..=100).contains(&value),
        //     "Guess value must be between 1 and 100 got {}",
        //     value
        // );
        if (1..=100).contains(&value) {
            panic!("Expected a value between 1 and 100. got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    recoverables_err1();
    recoverables_err2();
    let username = read_user_name_frm_file().unwrap_or("Error".to_string());
    println!("username: {username}",);

    error_info();

    let last_char =
        last_char_of_firstline("this is cool\nIlike it\n").unwrap_or(Default::default());
    println!("last char: {last_char}");
    let _greeting_file = File::open("./src/hello.txt")?;
    Ok(())
}
