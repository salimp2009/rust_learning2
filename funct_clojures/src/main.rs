#![allow(unused)]

use std::mem::{size_of, size_of_val};

pub fn bar<T>() {
    size_of::<T>();
}

// function pointer
pub fn baz(f: fn()) {
    // add code here
    println!("size of f: {}", size_of_val(&f));
}

pub fn foo() -> usize {
    5
}

pub fn quox<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

pub fn function_items() {
    // type of x is function item
    let x = bar::<usize>;
    // x = bar::<isize>;
    // function items are zero sized
    println!("size of funct item: {}", std::mem::size_of_val(&x));
    //function item coerces to fn pointer
    // and since funct item is coerced into func pointer
    // size of it becomes size of a pointer (8 bytes here)
    // function pointers implement all 3 Fn, FnMut, FnOnce
    baz(x);
    baz(bar::<usize>);
    baz(bar::<isize>);
    quox(bar::<usize>);

    let fnptr: fn() = || println!("function pointer");
    baz(fnptr);
}

pub fn consume_return<F>(func: F)
where
    F: FnOnce() -> Vec<i32>,
{
    println!("Consumed: {:?}", func())
}

pub fn consumables() {
    let list = vec![1, 2, 3];
    let consumed = move || list;
    consume_return(consumed);
    // consumed cannot be recalled again
    // FnOnce requires the ownership of the captured variable
    // since the clojure is moved the captured variable is consumed
    // if it is a type like i32 or anything that implement copy then it is copied
    // consume_return(consumed);
    // println!("{:#?}", list);
}
pub fn baz2<F>(f: F, a: i32, b: i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    // add code here
    f(a, b)
}

pub fn closures() {
    let func = |x: i32, y: i32| x + y;
    println!("x + y: {}", baz2(func, 2, 4));
}

fn main() {
    function_items();
    consumables();
    closures();
}
