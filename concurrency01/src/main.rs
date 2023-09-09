use std::{error::Error, sync::mpsc, sync::mpsc::SendError, thread, time::Duration};

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
pub fn channels_basics() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || -> Result<(), SendError<String>> {
        let value = String::from("hi channels");
        println!("sending channel data: {}", value);
        tx.send(value)?;
        Ok(())
    });
    let received = rx.recv()?;
    println!("receiver channel got: {}", received);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    thread_handle();
    // thread_spawn();
    channels_basics()?;
    Ok(())
}
