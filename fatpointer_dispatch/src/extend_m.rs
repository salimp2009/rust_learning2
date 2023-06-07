#![allow(dead_code)]
use std::{fmt::Display, iter::Extend};

#[derive(Debug)]
struct MyVec<T>(Vec<T>);

impl<T> MyVec<T> {
    fn new() -> MyVec<T> {
        // MyVec::<T>(Vec::<T>::new())
        // the type T is interfered since the return type have type T
        MyVec(Vec::new())
    }

    fn add(&mut self, elem: T) {
        self.0.push(elem);
    }
}

impl<T> Extend<T> for MyVec<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().for_each(|elem| self.add(elem));
        // for elem in iter {
        //     self.add(elem);
        // }
    }
}

pub fn extend_myvec() {
    let mut myvec: MyVec<i32> = MyVec::new();
    myvec.extend([1, 2, 3, 4, 6].into_iter());
    println!("myvec: {myvec:#?}");
}

pub fn extend_example() {
    let mut messages = String::from("the first three letters are: ");
    messages.extend(['a', 'b', 'c'].iter());
    println!("messages extended: {messages:?} ");
}

pub fn it<T>(v: &mut dyn Iterator<Item = T>)
where
    T: Display,
{
    let _x = v.next();
    v.into_iter()
        .for_each(|item| println!("trait object Iterator: {}", item));
}

// pub fn extend_example2(v: &mut dyn Extend<bool>) {
//     v.extend(std::iter::once(true));
// }

// dynamic custom type
pub struct Foo {
    f: bool,
    x: bool,
    t: [u8],
}

fn foo(f: &dyn Fn()) {
    f();
}

fn bar<T>(f: &dyn Fn() -> T) -> T {
    f()
}

fn bar2(f: fn()) {
    f()
}

#[cfg(test)]
mod extend_tests {
    use super::*;
    #[test]
    fn extend_test1() {
        extend_example();
        let x = 5;
        let y = 5;
        println!("x == y: {}", x == y);
    }

    #[test]
    fn test_myvec() {
        extend_myvec();
    }

    #[test]
    fn test_it() {
        it(&mut [true, false, true].into_iter());
    }

    #[test]
    fn test_dynfunc() {
        let x = String::from("yello");
        foo(&|| println!("{}", &x));
        println!("{}", bar(&|| &x));
        println!("x: {x}");
        // below doesn't work since it expects a function pointer
        // let testfn = &|| println!("{}", &x);
        // bar2(&testfn);
    }
}
