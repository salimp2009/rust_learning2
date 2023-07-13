pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

pub fn lifetimes_functions1() {
    let string1 = String::from("looser");
    let string2 = "winner";
    let result = longest(string1.as_str(), string2);
    println!("longest string is {result}");
}
pub fn lifetimes_functions2() {
    let string1 = String::from("looser");
    {
        let string2 = "winner";
        let result = longest(string1.as_str(), string2);
        println!("longest string is {result}");
    }
}

fn main() {
    lifetimes_functions1();
    lifetimes_functions2();
}
