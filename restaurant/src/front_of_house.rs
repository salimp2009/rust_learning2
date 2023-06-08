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
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    hosting::add_to_waitlist();
    serving::take_order();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
