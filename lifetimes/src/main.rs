use std::fmt::Display;

pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
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
    let string1 = String::from("looser longer :)");
    let result;
    {
        let string2 = "winner";
        result = longest(string1.as_str(), string2);
    }
    println!("longest string is {result}");
}
pub fn struct_lifetimes() {
    let novel = String::from("like to swim. in hot weathers in Istanbul :)");
    let first_sentence = novel.split('.').next().expect("no seperator '.' found!");
    let excerpt = ImportanteExcerpt { a: first_sentence };

    println!("first sentence from novel : {}", excerpt.a);
}

pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Big news : {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportanteExcerpt<'a> {
    a: &'a str,
}
fn main() {
    lifetimes_functions1();
    lifetimes_functions2();
    struct_lifetimes();
}
