use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Weak},
};

use futures;
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

pub trait Observer3: Send + Sync {
    type Subject;
    type Output;
    fn observe3<'a>(
        &'a self,
        subject: &'a Self::Subject,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + 'a + Send>>;
}

impl Observer3 for MyObserver {
    type Subject = Subject;
    type Output = ();

    #[allow(clippy::manual_async_fn)]
    fn observe3<'a>(
        &'a self,
        _subject: &'a Self::Subject,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + 'a + Send>> {
        Box::pin(async {
            println!("Observer3 async trait:before sleep");
            use ::tokio::time::{sleep, Duration};
            sleep(Duration::from_millis(1000)).await;
            println!("Observer3 async trait:after sleep");
        })
    }
    // add code here
}

pub trait Observable {
    type Observer;
    fn update<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>>;
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
}

pub struct MyObserver3 {
    name: String,
}
impl MyObserver3 {
    pub fn new(name: &str) -> Arc<Self> {
        Arc::new(Self { name: name.into() })
    }
}
impl Observer3 for MyObserver3 {
    type Subject = Subject3;
    type Output = ();

    #[allow(clippy::manual_async_fn)]
    fn observe3<'a>(
        &'a self,
        subject: &'a Self::Subject,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + 'a + Send>> {
        Box::pin(async {
            println!(
                "observed3 subject3 with state {:?} in {}",
                subject.state(),
                self.name
            );
        })
    }
    // add code here
}

pub struct Subject3 {
    observers: Vec<Weak<dyn Observer3<Subject = Self, Output = ()>>>,
    state: String,
}

impl Subject3 {
    pub fn new(state: &str) -> Self {
        Self {
            observers: vec![],
            state: state.into(),
        }
    }

    pub fn state(&self) -> &str {
        self.state.as_ref()
    }
}

impl Observable for Subject3 {
    type Observer = Arc<dyn Observer3<Subject = Self, Output = ()>>;

    fn update<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>> {
        let observers: Vec<_> = self
            .observers
            .iter()
            .flat_map(|obsrvr| obsrvr.upgrade())
            .collect();

        Box::pin(async move {
            futures::future::join_all(observers.iter().map(|o| o.observe3(self))).await;
        })
    }

    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }

    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(|f| !f.ptr_eq(&Arc::downgrade(&observer)));
    }
    // add code here
}
