#![allow(unused)]

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
    F: FnMut(),
{
}

// pub trait Fnm<Args>: FnOnce(Args) {
//     fn call(&self, args: Args) -> Self::Output;
// }

// impl<A, F> Fnm<A> for F
// where
//     F: ~const Fnm<A> + ?Sized,
//     A: std::marker::Tuple,
// {
//     fn call(&self, args: A) -> Self::Output {
//         *self.call(args)
//     }
// }

// impl<F> FnOnce() for F
// where
//     F: FnMut(),
// {
//     pub fn call_once(&self) -> Self::Output {
//         F::call_once()
//     }
// }

pub fn consume_return<F>(func: F)
where
    F: FnOnce() -> Vec<i32>,
{
    println!("Consumed: {:?}", func())
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

    let fnptr: fn() = || println!("function pointer");
    baz(fnptr);
    let list = vec![1, 2, 3];
    let consumed = move || list;
    consume_return(consumed);
    // consumed cannot be recalled again
    // since it moves the captured variable
    // consume_return(consumed);
    // println!("{:#?}", list);
}
