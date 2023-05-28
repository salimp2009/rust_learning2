use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn reverse_words(str: &str) -> String {
    str.split(' ')
        .map(|word| word.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let reversed = reverse_words("saitos mutos");
    println!("reversed: {reversed}");

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).unwrap_or_else(|error| {
            println!("{error}");
            error.to_string().len()
        });

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Jackspot!! Congratulations!");
                break;
            }
        }

        println!("secret_number: {}", secret_number);
        println!("your guess is {guess}")
        // match io::stdin().read_line(&mut guess) {
        //     Ok(n) => {
        //         println!("{n} bytes read");
        //         println!("{guess}");
        //     }
        //     Err(error) => println!("error: {error}"),
        // }
    }
}
