pub fn print_string(s: String) {
    println!("print_string: {}", s);
}

pub fn print_str(s: &str) {
    println!("print_str: {}", s);
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
}
