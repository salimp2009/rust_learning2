pub fn is_string(val: &dyn std::any::Any) {
    if val.is::<String>() {
        println!("val is String");
    } else {
        println!("val is not String");
    }
}

pub fn main() {
    let mut empty_vec = Vec::<i32>::new();
    (0..10).for_each(|val| {
        println!(
            "empty_vec has {} elems with capacity {}",
            empty_vec.len(),
            empty_vec.capacity()
        );
        empty_vec.push(val);
    });
    empty_vec.shrink_to_fit();
    println!("empty_vec cap after shrink {}", empty_vec.capacity());
    let myval = "10".to_owned();
    is_string(&myval);
    println!("{}", myval);
}
