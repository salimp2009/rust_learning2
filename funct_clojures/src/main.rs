use std::mem::{size_of, size_of_val};

pub fn bar<T>() {
    size_of::<T>();
}

pub fn baz(f: fn()) {
    // add code here
    println!("size of f: {}", size_of_val(&f));
}

pub fn foo() -> usize {
    5
}

pub fn quox<F>(f: F)
where
    F: Fn(),
{
}

fn main() {
    println!("Hello, world!");
    // type of x is function item
    let x = bar::<usize>;
    // x = bar::<isize>;
    // function items are zero sized
    println!("size of funct item: {}", std::mem::size_of_val(&x));
    //function item coerces to fn pointer
    // and since funct item is coerced into func pointer
    // size of it becomes size of a pointer (8 bytes here)
    baz(x);
    baz(bar::<usize>);
    baz(bar::<isize>);
    quox(bar::<usize>);
}
