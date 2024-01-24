use async_runtimes::observer::{self, *};
use std::time;
use tokio::runtime::Handle;
use tokio::task::{JoinError, JoinHandle};
pub fn tokio_builder() {
    let duration = time::Duration::from_secs(1);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    rt.block_on(async {
        tokio::time::sleep(duration).await;
        println!("Hello Tokio")
    });
}

pub fn not_async_function() -> JoinHandle<()> {
    tokio::task::spawn(async {
        println!("spawing from not_async");
    })
}

pub fn not_async_function2(handle: Handle) {
    handle.block_on(async {
        println!("second print statement");
    })
}
// this is non_blocking now due sleep(..).await
async fn sleep_1s_blocking(task: &str) {
    use tokio::time::{sleep, Duration};
    println!("Entering sleep_1s_non_blocking({task})");
    sleep(Duration::from_secs(1)).await;
    println!("Returning from sleep_1s_non_blocking({task})");
}

fn returns_option() -> Result<i32, ()> {
    Ok(5)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<(), JoinError> {
    let subject = Subject;
    let observer = MyObserver;
    observer.observe(&subject).await;

    println!("Test 1: Run 2 async task sequentially");
    sleep_1s_blocking("Task 1").await;
    sleep_1s_blocking("Task 2").await;

    println!("Test 2: Run 2 async task concurrently (same thread)");
    tokio::join!(sleep_1s_blocking("Task 3"), sleep_1s_blocking("Task 4"));

    println!("Test 3: Run 2 async tasks in parallel");
    let (_, _) = tokio::join!(
        tokio::spawn(sleep_1s_blocking("Task 5")),
        tokio::spawn(sleep_1s_blocking("Task 6"))
    );

    let Ok(res) = returns_option() else {
        unreachable!()
    };
    println!("value : {}", res);

    // let duration = time::Duration::from_secs(1);
    // tokio::time::sleep(duration).await;
    // // tokio_builder();

    // this prints first since it is awaited
    // async {
    //     println!("This line prints first");
    // }
    // .await;

    // let handle = Handle::current();

    // std::thread::spawn(move || {
    //     not_async_function2(handle);
    // });

    // // this may or may not run since there is no await
    // tokio::task::spawn(async {
    //     println!("This line prints sometimes, depending on how quick it runs")
    // });

    // not_async_function().await?;
    // println!("tokio is running from main runtime");
    Ok(())
}
