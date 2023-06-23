use std::future::Future;

use futures::executor::block_on;

// #![allow(dead_code)]
pub async fn foo1() -> usize {
    println!("foo1 is called");
    0
}

#[allow(clippy::manual_async_fn)]
pub fn foo2() -> impl Future<Output = usize> {
    async {
        println!("printing foo1 in foo2 soon");
        foo1().await;
        println!("foo2");
        0
    }
}

pub async fn foo3() -> usize {
    foo1().await
}

async fn hello_world() {
    println!("hello async");
}

pub fn main() {
    println!("hello from main");
    let _y = foo2();
    let _z = async {
        let x = foo3().await;
        println!("x: {:#?}", x);
        x
    };
    let future = hello_world();
    block_on(future);
    block_on(foo1());
    block_on(foo2());
    block_on(foo3());
}
