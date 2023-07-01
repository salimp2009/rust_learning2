use std::{collections::HashMap, default::Default};

pub fn vector_1() {
    let v = vec![1, 2, 3, 4];
    let third_element = &v[2];
    let third_element_alt = &v[2];
    assert_eq!(third_element, third_element_alt);
    println!("vector1: {}", third_element);
    println!(
        "3rd elem == 3r elem_alt: {}",
        third_element == third_element_alt
    );
    println!("100th elem of vector1: {:#?}", v.get(100).take());
}

pub fn vector_2() {
    let mut v: Vec<_> = vec![1, 2, 3, 4];
    assert_eq!(v, &[1, 2, 3, 4]);
    let third = v.get(2);
    // v.push(5);
    println!("vec2: {:#?}", third);
}

pub fn vector_iterate() {
    let mut v = vec![1, 2, 3, 4];
    println!("vector_iterate");
    v.iter()
        .map(|elem| elem * 2)
        .for_each(|elem| println!("{}", elem));

    println!("original vector_iterate: {:#?}", v);

    v.iter_mut().for_each(|elem| {
        *elem *= 2;
    });
    println!("modified vector_iterate: {:#?}", v);
}

pub fn flatmap_filter() {
    (0..5)
        .flat_map(|x| x * 100..x * 110)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| println!("{i}:{x}"));
}

pub fn vector_with_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(20),
        SpreadsheetCell::Text("blue".to_string()),
        SpreadsheetCell::Float(20.55),
    ];
    println!("row: {:#?}", row);
}

pub fn basic_strings() {
    let mut s1 = String::from("fooy");
    let s2 = "yoooww";
    s1.push_str(s2);
    println!("s2: {s2}");
    let s3 = s1 + s2;
    assert_eq!(s3, "fooyyooowwyoooww");
    println!("s3: {s3}");
    println!("s2: {s2}");
    // s1 is moved into s3; it is not valid anymore!
    // println!("s1: {s1}");
}

pub fn string_get() {
    let mut v = String::from("hello");
    assert_eq!(Some("he"), v.get(0..2));
    assert_eq!(Some("he"), v.get_mut(0..2).map(|elem| &*elem));
    assert_eq!("he", &v[0..2]);
    let hello = "Здравствуйте";
    let answer = hello.as_bytes();
    println!("answer: {answer:#?}");
    "Зд".chars().for_each(|elem| println!("{elem}"));
}

pub fn hashmap_basic1() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    let default_val = Default::default();
    let value = scores.get("Blue").unwrap_or(&default_val);
    let value2 = scores.get("Blue").copied().unwrap_or(default_val);
    println!("value: {}", value);
    println!("value2: {value2}");
}

fn main() {
    vector_1();
    vector_2();
    vector_iterate();
    flatmap_filter();
    vector_with_enum();
    basic_strings();
    string_get();
    hashmap_basic1();
}
