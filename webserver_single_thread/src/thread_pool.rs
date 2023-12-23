use std::thread;

pub struct ThreadPool {
    pub threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut threads = Vec::with_capacity(size);

        ThreadPool { threads }
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
