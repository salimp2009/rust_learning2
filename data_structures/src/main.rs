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

pub fn check_primitives() {
    let value = 0u8;
    println!(
        "value: {}, length={}, length of u8={}",
        value,
        std::mem::size_of_val(&value),
        std::mem::size_of::<u8>()
    );
    let value = 0u32;
    println!(
        "value: {}, length={}, length of u32={}",
        value,
        std::mem::size_of_val(&value),
        std::mem::size_of::<u32>()
    );
    let value = 0u128;
    println!(
        "value: {}, length={}, length of u128={}",
        value,
        std::mem::size_of_val(&value),
        std::mem::size_of::<u128>()
    );
    println!("Binary (base 2)         0b1111_1111={}", 0b1111_1111);
    println!("Octal (base 8)          0o1111_1111={}", 0o1111_1111);
    println!("Decimal (base 10)       1111_1111={}", 1111_1111);
    println!("Hexadecimal (base 16)   0x1111_1111={}", 0x1111_1111);
    println!("Byte literal            b'A'={}", b'A');
}

struct EmptyStruct {}
struct AnotherEmptyStruct;

#[derive(Debug, Clone, Default)]
struct DebuggableStruct {
    string: String,
    number: i32,
}
impl DebuggableStruct {
    pub fn increment_number(&mut self) {
        self.number += 1;
    }

    pub fn incremented_number(mut self) -> Self {
        self.number += 1;
        self
    }
}

#[derive(Debug, PartialEq)]
enum JapaneseDogs {
    AkitaKen,
    HokkaidoInu,
    KaiKen,
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
    check_primitives();

    println!(
        "EmptyStruct size : {}, {}",
        std::mem::size_of_val(&EmptyStruct {}),
        std::mem::size_of::<EmptyStruct>()
    );
    let mut debuggable_struct = DebuggableStruct::default();
    debuggable_struct.increment_number();
    println!(
        "debuggable_struct: {:?}",
        debuggable_struct.incremented_number()
    );
    println!(
        "using enums JapaneseDogs: {}",
        JapaneseDogs::AkitaKen as u32
    );
}
