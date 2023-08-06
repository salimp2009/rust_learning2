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
}
