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

pub fn channels_multimesg() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        // let vals = vec![
        //     String::from("hi"),
        //     String::from("from"),
        //     String::from("the"),
        //     String::from("thread"),
        // ];

        for val in (["hi", "from", "the", "thread"]).iter() {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx.iter() {
        println!("received : {}", received);
    }
}
pub fn channels_multiproducer() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        for val in (["hi", "from", "the", "thread"]).iter() {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in (["more", "messages", "Dido", "Semos"]).iter() {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx.iter() {
        println!("received : {}", received);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // thread_handle();
    // thread_spawn();
    // channels_basics()?;
    // channels_multimesg();
    channels_multiproducer();
    Ok(())
}
