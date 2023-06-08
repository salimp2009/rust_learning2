use std::mem::{size_of, size_of_val};

pub fn bar<T>() -> usize {
    size_of::<T>()
}

pub fn baz(f: fn() -> usize) {
    // add code here
    println!("size of f: {}", size_of_val(&f));
}

pub fn foo() -> usize {
    5
}

fn main() {
    println!("Hello, world!");
    // type of x is function item
    let x = bar::<usize>;
    // x = bar::<isize>;
    println!("size of funct item: {}", std::mem::size_of_val(&x));
    //function item coerces to fn pointer
    baz(x);
    baz(bar::<usize>);
    baz(bar::<isize>);
}
