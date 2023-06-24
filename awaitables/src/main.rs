#![allow(dead_code)]
use std::{future::Future, time};

// use futures::executor::block_on;

use tokio::task::JoinHandle;
pub async fn foo1() -> usize {
    println!("foo1 is called");
    tokio::time::sleep(time::Duration::from_secs(15)).await;
    println!("waiting on foo1 is complete!");

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
    println!("x2 from foo2: {}", x2);
    0
}

pub fn not_an_async_function() -> JoinHandle<()> {
    tokio::task::spawn(async {
        println!("tokio task printing!");
    })
}

#[tokio::main]
async fn main() {
    let zz = foo3().await;
    println!("foo3 from main zz {}", zz);
    async {
        println!("hello from main");
    }
    .await;
    not_an_async_function().await.ok();
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
