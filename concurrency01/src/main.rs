use std::{thread, time::Duration};
pub fn thread_handle() {
    let handle = thread::spawn(|| {
        (1..10).for_each(|num| {
            println!(
                "hi the number is {:?} from thread: {:?}",
                num,
                thread::current().id()
            );
            thread::sleep(Duration::from_millis(5));
        })
    });

    // handle.join().unwrap();

    (1..10).for_each(|num| {
        println!(
            "from thread_handle number is {:?}  id: {:?}",
            num,
            thread::current().id()
        );
        thread::sleep(Duration::from_millis(5));
    });
    handle.join().unwrap();
    println!("from thread_handle  id: {:?}", thread::current().id());
}

pub fn thread_spawn() {
    thread::spawn(|| {
        (1..10).for_each(|num| {
            println!(
                "hi the number is {:?} from thread: {:?}",
                num,
                thread::current().id()
            );
            thread::sleep(Duration::from_millis(5));
        })
    });
    (1..10).for_each(|num| {
        println!(
            "from thread_spawn number is {:?}  id: {:?}",
            num,
            thread::current().id()
        );
        thread::sleep(Duration::from_millis(5));
    });
}
fn main() {
    thread_handle();
    // thread_spawn();
}
