use std::{thread, time::Duration};

use tshirt_co::inventory::{self, ShirtColor};

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
    let s = example_closure("hello".to_string());
    // this wont work since the type for input in closure is set to String in above code
    // let n = example_closure(5);
}
