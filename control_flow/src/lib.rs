#[derive(Debug)]
pub enum UsStates {
    Alabama,
    Alaska,
    Seattle,
    California,
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => match state {
            UsStates::California => 25,
            UsStates::Alabama => 15,
            _ => {
                println!("State quarter from {:?}", state);
                25
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cents_check_() {
        assert_eq!(value_in_cents(Coin::Quarter(UsStates::Seattle)), 25);
    }
}
