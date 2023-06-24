#![allow(dead_code)]
use futures::executor::block_on;
use std::{future::Future, time};
use tokio::task::JoinHandle;

pub async fn foo1() -> usize {
    println!("foo1 is called");
    tokio::time::sleep(time::Duration::from_secs(5)).await;
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

#[derive(Debug)]
pub struct Song {
    songname: String,
}

pub async fn learn_song() -> Song {
    let song1 = Song {
        songname: "SingaSong".to_string(),
    };
    println!("learning song {:#?}", song1);
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    song1
}

pub async fn sing_song(song: Song) {
    println!("Singing {song:#?}")
}

pub async fn dance() {
    println!("dancing :)");
}

pub async fn learn_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

pub async fn async_main() {
    let f1 = learn_sing();
    let f2 = dance();
    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

#[tokio::main]
async fn main() {
    block_on(async_main());
    let zz = foo3().await;
    println!("foo3 from main zz {}", zz);
    async {
        println!("hello from main");
    }
    .await;
    not_an_async_function().await.ok();
}
