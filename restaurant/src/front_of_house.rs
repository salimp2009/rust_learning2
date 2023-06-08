#![allow(dead_code, unused)]
pub mod hosting {
    pub fn add_to_waitlist() {}

    pub fn seat_at_table() {}
}

pub mod serving {
    use super::hosting::seat_at_table;

    pub fn take_order() {
        serve_order();
        take_payment();
    }

    fn serve_order() {
        seat_at_table()
    }

    fn take_payment() {}
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer_toast("Rye");
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    meal.toast = String::from("Wheat");
    println!("that is a great {} toast!", meal.toast);
    println!("Orders: {order1:?}, {order2:?}");
}

fn deliver_order() {}

mod back_of_house {
    // all fields in enum is public
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        // if not identified with pub this field is private
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer_toast(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apple"),
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_restaurant() {
        eat_at_restaurant();
    }
}
