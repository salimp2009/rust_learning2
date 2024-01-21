use std::time;
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

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    // let duration = time::Duration::from_secs(1);
    // tokio::time::sleep(duration).await;
    // // tokio_builder();

    // this prints first since it is awaited
    async {
        println!("This line prints first");
    }
    .await;

    // this may or may not run since there is no await
    tokio::task::spawn(async {
        println!("This line prints sometimes, depending on how quick it runs")
    });

    // not_async_function().await?;
    println!("tokio is running from main runtime");
    Ok(())
}
