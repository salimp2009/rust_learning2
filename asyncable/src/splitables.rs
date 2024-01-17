use std::future;

use futures::{select, FutureExt};
use pin_utils::pin_mut;

pub async fn task_one() -> future::Ready<i32> {
    println!("tasking one async");
    future::ready(2)
}

pub async fn task_two() -> future::Pending<()> {
    println!("tasking two async");
    future::pending()
}

pub async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();
    pin_mut!(t1, t2);
    loop {
        select! {
            a = t1 => println!("task one completed first! => result: {:#?}",a),
            b = t2 => println!("task two completed first!" ),
            complete => break,
            default => unreachable!(),
        };
    }
}
