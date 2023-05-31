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

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

pub fn first_word2(s: &str) -> &str {
    let words = s.split(' ').collect::<Vec<_>>();
    words[0]
}

pub fn first_word3(s: &str) -> &str {
    let words = s.split_whitespace().collect::<Vec<_>>();
    words[0]
}

pub fn first_word_alt(s: &str) -> &str {
    let mut iterator = s.split_whitespace();
    if let Some(first_word) = iterator.next() {
        return first_word;
    };
    s
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

    let mut s = String::from("helloЗд Rustiiii");
    let word1 = first_word(&s);

    // illegal use of mutable since it is borrowed and used after
    // s.clear();
    println!("word1: {word1}");

    // when using slice indices, if not Latin characters are used
    // it might cause problems to due char boundary
    // since those are encoded with 2 bytes
    let slice1: &str = &"hello  world".to_string()[0..6];
    println!("slice1: {slice1}");

    let word1_alt = first_word_alt(&s);
    println!("word2 : {}", word1_alt);

    let word1_alt2 = first_word2(&s);
    println!("word1_alt2: {word1_alt2}");
}
