use std::{thread, time::Duration};

use tshirt_co::inventory::{self, ShirtColor};

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn closures_capture_byref() {
    let list = vec![1, 2, 3];
    println!("Before the closure {:?}", list);

    let only_borrows = || println!("inside the closure {:?}", list);
    println!("Before calling the closure {:?}", list);
    only_borrows();
    println!("After calling the closure {:?}", list);
}

pub fn closure_with_move() {
    let list = vec![1, 2, 3];
    println!("Before the closure_moves {:?}", list);

    thread::spawn(move || println!("inside the closure_moves {:?}", list))
        .join()
        .unwrap();
    // list is moved therefore illegal borrow below
    // println!("after move {:?}", list);
}

fn main() {
    let store = inventory::Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1.unwrap(),
        giveaway
    );

    let user_pref2 = None;
    let giveaway = store.giveaway(user_pref2);
    println!("User with no preference gets {giveaway:?}");

    // adding type annotation for closures is optional
    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let example_closure = |x| x;
    let _s = example_closure("hello".to_string());
    // this wont work since the type for input in closure is set to String in above code
    // let n = example_closure(5);

    let mut list_rectangles = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    let mut num_sort_operations = 0;

    list_rectangles.sort_by_cached_key(|elem| {
        num_sort_operations += 1;
        elem.width
    });
    println!("{:?} sorted in {num_sort_operations}", list_rectangles);
    closures_capture_byref();
    closure_with_move();
}
