#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ShirtColor {
    Red,
    Blue,
}
/// Creates Inventory
///
/// # Examples
/// ```
///    use tshirt_co::inventory::{self, ShirtColor};
///    let store = inventory::Inventory {
///        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
///    };
///
///    let user_pref1 = Some(ShirtColor::Red);
///    let giveaway = store.giveaway(user_pref1);
///    println!(
///        "The user with preference {:?} gets {:?}",
///        user_pref1.unwrap(),
///        giveaway
///    );
/// ```
#[derive(Debug, PartialEq)]
pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        self.shirts.iter().for_each(|color| match color {
            ShirtColor::Red => num_red += 1,
            ShirtColor::Blue => num_blue += 1,
        });
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
