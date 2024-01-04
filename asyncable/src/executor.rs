#![allow(dead_code, unused_imports)]

use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

use std::{
    cell::UnsafeCell,
    future::Future,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::Context,
    time::Duration,
};

use crate::timerfuture::TimerFuture;

/// Task executor that receives tasks off of a channel and runs them.
pub struct Executor {
    read_queue: Receiver<Arc<Task>>,
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

pub struct Task {
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need to use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    // future: UnsafeCell<Option<BoxFuture<'static, ()>>>,
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    /// Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}
