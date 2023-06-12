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

pub fn quox<F>(f: &F)
where
    F: Fn(),
{
    f();
}

pub fn quox2<F>(f: &mut F)
where
    F: FnMut(),
{
    f();
}

pub fn quox3<F>(mut f: F)
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
    quox(&bar::<usize>);

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
    let func = || ();
    quox(&func);
    let mut z = String::from("closures");
    // closures that captures cannot be coerced to fn() (function pointers)
    let mut func2 = move || {
        // let _x = z;
        z.clear();
        // drop requires z to moved therefore closure becomes FnOnce but
        // quox2 requires FnMut trait
        // drop(z);
        // println!("z: {z}");
    };
    // func2 cannot be coerced to fn() since it captures variable from environment
    // baz(func2);
    // quox(&func2);

    // z is moved into closure due to move keyword therefore z(String)
    // cannot be used
    // println!("z: {z}");
    quox2(&mut func2);

    let mut z2 = String::from("closures");
    let func3 = || {
        // let _x = z;
        z2.clear();
        // println!("z: {z}");
    };
    quox3(func3);

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

pub fn make_fn() -> impl FnOnce() {
    let z = String::from("hi from make function");
    move || println!("z from make_fn: {z}")
}

pub fn closures() {
    let func = |x: i32, y: i32| x + y;
    println!("x + y: {}", baz2(func, 2, 4));
}

fn main() {
    function_items();
    consumables();
    closures();
    let func2 = make_fn();
    let f2: Box<dyn FnOnce()> = Box::new(func2);
    f2();
}
