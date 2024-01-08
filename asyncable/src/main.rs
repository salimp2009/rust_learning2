#![allow(dead_code, unused_variables)]

use std::{future::Future, time::Duration};

use asyncable::{executor::new_executor_and_spawner, timerfuture::TimerFuture};
use futures::executor::block_on;

#[derive(Debug)]
struct Song {
    title: String,
    id: i32,
}

async fn learn_song() -> Song {
    Song {
        title: "dabadabadoo".to_string(),
        id: 5,
    }
}

async fn sing_song(song: Song) {
    println!("Singing: {}, {}", song.title, song.id);
}

pub async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}
pub async fn using_futures() {
    println!("hello async");
}

pub async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = using_futures();
    futures::join!(f1, f2);
}

#[allow(clippy::manual_async_fn)]
pub fn foo_lifetime<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}

pub fn bad() -> impl Future<Output = u8> + 'static {
    /// original example
    // let x = 5;
    // foo_lifetime(&x)

    // revised to make it compile
    const X: u8 = 5;
    foo_lifetime(&X)
}

/// By moving the argument into the async block,
/// extend its lifetime to match that of the Future
/// returned from the call to good.
#[allow(clippy::manual_async_fn)]
fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        foo_lifetime(&x).await
    }
}

/// `async` block:
///
/// Multiple different `async` blocks can access the same local variable
/// so long as they're executed within the variable's scope
pub async fn blocks() {
    let my_string = "salitos".to_string();

    let future_one = async {
        println!("future one: {}", my_string);
    };

    let future_two = async {
        println!("future one: {}", my_string);
    };
}

pub async fn move_blocks() {
    let my_string = "salitos".to_string();

    let future_one = async move {
        println!("future one: {}", my_string);
    };

    // /// this will not compile since my_string is moved above
    // let future_two = async {
    //     println!("future one: {}", my_string);
    // };
}

fn main() {
    let _future = using_futures();
    block_on(using_futures());
    block_on(async_main());
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer
    spawner.spawn(async {
        println!("our executor & future says hello");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done after TimerFuture");
    });

    spawner.spawn(async {
        println!("our executor runs async_main");
        TimerFuture::new(Duration::new(2, 0)).await;
        async_main().await;
        println!("done after async_main & TimerFuture");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}
