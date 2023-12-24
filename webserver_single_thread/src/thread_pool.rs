use std::{
    sync::mpsc,
    thread::{self},
};

struct Job;

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, _receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        (0..size).for_each(|id| workers.push(Worker::new(id)));
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // let mut count = self.size;
        // count -= 1;

        // if count > 0 {
        //     std::thread::spawn(f);
        // }
    }
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Self {
        Worker {
            id,
            thread: thread::spawn(|| {}),
        }
    }
}
