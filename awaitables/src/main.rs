#![allow(dead_code)]
use std::{future::Future, thread, time};

use futures::executor::block_on;

use tokio;
pub async fn foo1() -> usize {
    println!("foo1 is called");
    tokio::time::sleep(time::Duration::from_secs(8)).await;
    0
}

#[allow(clippy::manual_async_fn)]
pub fn foo2() -> impl Future<Output = usize> {
    async {
        println!("printing foo2");
        // tokio::time::sleep(time::Duration::from_secs(1)).await;
        0
    }
}

pub async fn foo3() -> usize {
    let x1 = foo1().await;
    let x2 = foo2().await;
    println!("x1 from foo1: {}", x1);
    println!("x2 from foo1: {}", x2);
    0
}

#[tokio::main]
async fn main() {
    let _zz = foo3().await;
    async {
        println!("hello from main");
    }
    .await;
    println!("foo3 from main zz {}", _zz);
    // let _y = foo2();
    // let _z = async {
    //     let x = foo3().await;
    //     println!("x: {:#?}", x);
    //     x
    // };
    // let future = hello_world();
    // block_on(future);
    // block_on(foo1());
    // block_on(foo2());
    // block_on(foo3());
}
