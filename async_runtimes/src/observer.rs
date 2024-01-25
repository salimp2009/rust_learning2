use std::{future::Future, pin::Pin};

pub struct Subject;
pub struct MyObserver;

unsafe impl Send for Subject {}
unsafe impl Send for MyObserver {}

#[allow(async_fn_in_trait)]
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
            sleep(Duration::from_millis(2000)).await;
            println!("using async trait Observer after sleep !!!");
        });
    }
}

pub trait Observer2 {
    type Subject;
    type Output;
    fn observe2(&self, subject: &Self::Subject) -> impl Future<Output = Self::Output> + Send;
}

impl Observer2 for MyObserver {
    type Subject = Subject;
    type Output = ();

    #[allow(clippy::manual_async_fn)]
    fn observe2(&self, _subject: &Self::Subject) -> impl Future<Output = Self::Output> + Send {
        async {
            println!("Observer2 async trait:before sleep");
            use ::tokio::time::{sleep, Duration};
            sleep(Duration::from_millis(1000)).await;
            println!("Observer2 async trait:after sleep");
        }
    }
    // add code here
}

pub trait Observer3 {
    type Subject;
    type Output;
    fn observe3(
        &self,
        subject: &Self::Subject,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + Send>>;
}

impl Observer3 for MyObserver {
    type Subject = Subject;
    type Output = ();

    #[allow(clippy::manual_async_fn)]
    fn observe3(
        &self,
        _subject: &Self::Subject,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + Send>> {
        Box::pin(async {
            println!("Observer3 async trait:before sleep");
            use ::tokio::time::{sleep, Duration};
            sleep(Duration::from_millis(1000)).await;
            println!("Observer3 async trait:after sleep");
        })
    }
    // add code here
}
