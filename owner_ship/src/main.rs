pub fn takes_ownership(s: String) {
    println!("take ownership: {s:?}")
}

pub fn takes_refences(s: &String) {
    println!("take refences: {s:?}")
}

pub fn takes_slice(s: &str) {
    println!("take slices: {s:?}")
}

pub fn gives_ownership() -> String {
    String::from("owner is yours")
}

pub fn takes_and_gives_ownership(a_string: String) -> String {
    a_string
}

pub fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1:{s1}, s2:{s2}");
    takes_ownership(s1);
    // illegal call due to s1 is move into function
    // println!("s1: {s1:?}");
    takes_refences(&s2);
    println!("s2: {s2}");

    takes_slice(&s2);
    println!("s2: {s2}");

    let s3 = gives_ownership();
    println!("gives_ownership: {s3}");
    let s4 = takes_and_gives_ownership(s3);
    // println!("s3 is moved: {s3}");
    println!("takes_and_gives_ownership: {s4}");
    let (s5, len) = calculate_length(s4);
    println!("string: {s5} and length: {len}");
}
