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

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    x.map(|value| value + 1)
}

pub fn config_max(value: Option<u8>) {
    if let Some(max) = value {
        println!("Maximum is configured to {max}");
    }
}

pub fn stupid(v: &[u8]) {
    println!("{:?}", &v[..3]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cents_check_() {
        assert_eq!(value_in_cents(Coin::Quarter(UsStates::Seattle)), 25);
    }

    #[test]
    fn plusone_check_() {
        let x = plus_one(Some(5));
        assert_eq!(x, Some(6));
        stupid([1, 2, 3, 4, 5, 6].as_ref());
        config_max(Some(3u8));
    }
}
