use tshirt_co::inventory::{self, ShirtColor};

fn main() {
    let store = inventory::Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue],
    };
}
