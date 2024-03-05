use metrohash::MetroBuildHasher;
use std::collections::HashMap;

pub fn print_string(s: String) {
    println!("print_string: {}", s);
}

pub fn print_str(s: &str) {
    println!("print_str: {}", s);
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct CompoundKey {
    name: String,
    value: i32,
}

pub fn main() {
    print_string(String::from("my String"));
    print_str(&String::from("my String"));
    print_str("my &str");

    let my_array = [0u8; 64];
    println!("my_array: {:#?}", my_array);
    let my_slice: &[u8] = &my_array;
    let (first_half, secnd_half) = (&my_array[0..=31], &my_array[32..my_array.len()]);
    println!(
        "first_half: {:#?}, secnd_half: {:#?}",
        first_half.len(),
        secnd_half.len()
    );

    let (first_half, secnd_half) = my_slice.split_at(32);
    println!(
        "first_half: {:#?}, secnd_half: {:#?}",
        first_half.len(),
        secnd_half.len()
    );

    let wordlist = "one, two, three";
    wordlist
        .split(',')
        .map(|word| word.trim())
        .for_each(|word| println!("word: {}", word));
    let mut my_vec = vec![1, 2, 3, 4];

    let vec_slize = my_vec.as_slice();
    // my_vec.resize(10, 5);
    println!("slice: {:?}", vec_slize);

    let mut map = HashMap::<String, String, MetroBuildHasher>::default();
    map.insert("hello".into(), "Hellomap".into());

    println!("Hashmap with MetroHasher: {:#?}", map);

    let mut map2 = HashMap::<CompoundKey, CompoundKey>::new();
    map2.insert(
        CompoundKey {
            name: "compound".into(),
            value: 5,
        },
        CompoundKey {
            name: "value".into(),
            value: 32,
        },
    );
    println!("Hashmap with CompoundKey: {:#?}", map2);
}
