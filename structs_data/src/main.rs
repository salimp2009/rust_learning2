#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
pub struct UserM<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

pub fn build_user(email: &str, username: &str) -> User {
    User {
        active: true,
        username: username.to_string(),
        email: email.to_string(),
        sign_in_count: 1,
    }
}

// Tuple structs without named fields
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

pub fn user_alt() {
    let user = UserM {
        active: true,
        username: "didemos",
        email: "zartzur@zartzurt.com",
        sign_in_count: 1,
    };
    println!("userM: {:#?}", user);
}

fn main() {
    let user1 = User {
        active: true,
        username: "salitos".to_string(),
        email: String::from("salitos@zartzort.com"),
        sign_in_count: 1,
    };
    println!("user1.email: {}", user1.email);

    let user2 = build_user("zalitos@fossfoss.com", "fossfoss");
    println!("user2: {:#?}", user2);

    let user3 = User {
        email: "yetanotherexample@sample.com".to_string(),
        ..user2
    };
    println!("user3: {:?}", user3);
    // illegal use of after the data is moved to user3
    // since String does not implement Copy
    // println!("user2: {:?}", user2);
    let black = Color(0, 0, 0);
    println!(
        "black color: r:{:?}, g:{:?}, b:{:?}",
        black.0, black.1, black.2
    );

    println!("black color: r:{:#?}", black);
    user_alt();
}
