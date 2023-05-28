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
}
