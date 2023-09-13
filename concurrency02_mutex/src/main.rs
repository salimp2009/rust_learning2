use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn mutex_api() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 10;
    }
    println!("Mutex m: {:?}", m);
}

pub fn sharing_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });
    println!("Result of counter: {}", *counter.lock().unwrap());
}

fn main() {
    mutex_api();
    sharing_mutex();
}
