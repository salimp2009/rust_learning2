#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    active: bool,
    username: String,
    email: String,
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

fn main() {
    let user1 = User {
        active: true,
        username: "salitos".to_string(),
        email: String::from("salitos@zartzort.com"),
        sign_in_count: 1,
    };
    println!("user1.email: {}", user1.email);

    let user2 = build_user("zalitos@fossfoss.com", "fossfoss");
    println!("user2: {:?}", user2);

    let user3 = User {
        email: "yetanotherexample@sample.com".to_string(),
        ..user2
    };
    println!("user3: {:?}", user3);
    // illegal use of after the data is moved to user3
    // since String does not implement Copy
    // println!("user2: {:?}", user2);
}
