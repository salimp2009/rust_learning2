use std::time::Duration;

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

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}
