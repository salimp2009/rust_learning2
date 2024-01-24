pub struct Subject;
pub struct MyObserver;

pub trait Observer {
    type Subject;
    async fn observe(&self, subject: &Self::Subject)
    where
        Subject: Send + Sync,
        Self: Send + Sync;
}

impl Observer for MyObserver {
    type Subject = Subject;
    async fn observe(&self, _subject: &Self::Subject) {
        tokio::spawn(async move {
            use ::tokio::time::{sleep, Duration};
            println!("using async trait Observer!!!");
            sleep(Duration::from_millis(1000)).await;
            println!("using async trait Observer after sleep !!!");
        });
    }
}
