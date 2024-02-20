use std::time::Instant;

pub fn is_string(val: &dyn std::any::Any) {
    if val.is::<String>() {
        println!("val is String");
    } else {
        println!("val is not String");
    }
}
pub fn vector_alloc_size() {
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
}

pub fn big_vec_test() {
    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    for i in big_vec {
        if i < 0 {
            println!("this never prints");
        }
    }
    println!("First loop with for took: {}", now.elapsed().as_secs_f32());

    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec.iter().for_each(|i| {
        if *i < 0 {
            println!("this never prints from iter");
        }
    });
    println!(
        "Second loop with iter took: {}",
        now.elapsed().as_secs_f32()
    );

    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec.into_iter().for_each(|i| {
        if i < 0 {
            println!("this never prints from iter");
        }
    });
    println!(
        "Third loop with into_iter took: {}",
        now.elapsed().as_secs_f32()
    );
}

pub fn main() {
    // vector_alloc_size();

    // let myval = "10".to_owned();
    // is_string(&myval);
    // println!("myval; is_string ? {}", myval);
    big_vec_test();
}
