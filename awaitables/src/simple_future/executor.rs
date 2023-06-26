use super::timer_future::TimerFuture;
use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use std::{
    future::Future,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::Context,
    time::Duration,
};

/// Task executor that receives tasks off of a channel and runs them.
struct Executorm {
    ready_queue: Receiver<Arc<Taskm>>,
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Debug)]
struct Spawnerm {
    task_sender: SyncSender<Arc<Taskm>>,
}

/// A future that can reschedule itself to be polled by an `Executor`.
struct Taskm {
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need to use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Taskm>>,
}

fn new_executor_spawner() -> (Executorm, Spawnerm) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executorm { ready_queue }, Spawnerm { task_sender })
}

impl Spawnerm {
    // spawns new futures
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Taskm {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

// to poll futures we need a Waker;
