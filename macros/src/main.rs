use macros::vecb;
use macros::HelloMacro;

#[derive(Debug)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello macro this is Pancake")
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    println!("nums : {:?}", nums);

    let nums = vecb!(1, 2, 3, 4);
    // let num3 = vecn!(1, 2, 3, 4, 5);
    println!("nums : {:?}", nums);
    Pancakes::hello_macro();
}
