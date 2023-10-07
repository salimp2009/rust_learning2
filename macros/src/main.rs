// use hello_macro_derive;
use macros::vecb;
use macros::HelloMacro;

#[derive(Debug)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello macro this is Pancake")
    }
}

#[derive(hello_macro_derive::HelloMacro)]
struct Pancakes2 {}

fn main() {
    let nums = vec![1, 2, 3, 4];
    println!("nums : {:?}", nums);

    let nums = vecb!(1, 2, 3, 4);
    println!("nums : {:?}", nums);
    Pancakes::hello_macro();
    Pancakes2::hello_macro();
}
