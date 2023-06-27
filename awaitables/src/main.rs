#![allow(dead_code)]
#![allow(unused_imports)]
use awaitables::simple_future::executor; /* ::{new_executor_spawnerm, Executorm} ;*/
use awaitables::simple_future::timer_future::TimerFuture;
use awaitables::simple_future::SimpleFuture;
use futures::executor::block_on;
use std::time::Duration;
use std::{future::Future, time};
use tokio::join;
use tokio::task::JoinHandle;

pub async fn foo1() -> usize {
    println!("foo1 is called");
    tokio::time::sleep(time::Duration::from_secs(5)).await;
    println!("waiting on foo1 is complete!");

    0
}

#[allow(clippy::manual_async_fn)]
pub fn foo2<'a>() -> impl Future<Output = usize> + 'a {
    let x = 5;
    async move {
        println!("printing foo2");
        // tokio::time::sleep(time::Duration::from_secs(1)).await;
        x
    }
}

pub async fn foo3() -> usize {
    let x1 = foo1().await;
    let x2 = foo2().await;
    println!("printing from foo3");
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

pub fn test_executorm() {
    let (executorm, spawnerm) = executor::new_executor_spawnerm();
    // Spawn a task to print before and after waiting on a timer.
    spawnerm.spawn(async {
        println!("whatis up spawner!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("exceturm is done!");
    });
    drop(spawnerm);
    executorm.run();
}
async fn foo5() -> u8 {
    5
}

pub async fn foo_blocks() {
    // value can be shared betwwen different async blocks
    let my_string = "foo_blocks".to_string();

    let future_one = async {
        println!("my_string: {my_string}");
    };

    let future_two = async {
        println!("my_string: {my_string}");
    };

    let ((), ()) = futures::join!(future_one, future_two);
}

#[tokio::main]
async fn main() {
    block_on(async_main());
    let zz = foo3().await;
    println!("foo3 from main zz {}", zz);
    async {
        println!("hello from main");
        println!("foo5 + 8: {:#?}", foo5().await + 8);
    }
    .await;
    not_an_async_function().await.ok();
    test_executorm();
    foo_blocks().await;
}
