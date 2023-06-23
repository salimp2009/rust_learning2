use std::future::Future;

// #![allow(dead_code)]
pub async fn foo1() -> usize {
    println!("foo1");
    foo2().await;
    0
}

pub fn foo2() -> impl Future<Output = usize> {
    async {
        println!("foo2");
        0
    }
}

pub async fn foo3() -> usize {
    let x = foo1().await;
    x
}

pub fn main() {
    println!("hello from main");
    // let x = foo3();
    // println!("x: {:#?}", x);
}
