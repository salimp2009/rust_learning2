use ::std::time::Instant;
use std::{char, i64};

use rand::Rng;
use rayon::prelude::*;
use regex::Regex;

pub fn generate_random_values(count: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut ret = Vec::with_capacity(count);

    (0..count).for_each(|_| ret.push(rng.gen_range(i64::MIN..i64::MAX)));
    ret
}

pub fn generate_random_strings(count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut ret = Vec::with_capacity(count);

    (0..50000).for_each(|_| {
        let mut rstring = String::new();
        rstring.push(char::from_u32(rng.gen_range(0..127)).unwrap_or(' '));
        ret.push(rstring);
        // ret.push(
        //     char::from_u32(rng.gen_range(0..127))
        //         .unwrap_or(' ')
        //         .to_string(),
        // );
    });
    ret
}

fn main() {
    let data = generate_random_values(1_000);

    let start = Instant::now();
    let sum = data
        .iter()
        .map(|num| num.wrapping_mul(*num))
        .reduce(|a: i64, b: i64| a.wrapping_add(b));
    let finish = Instant::now() - start;
    println!(
        "Summing squares without Rayon took: {}s, sum: {}",
        finish.as_secs_f64(),
        sum.unwrap()
    );
    // using parallel iterators
    let start = Instant::now();
    let sum = data
        .par_iter()
        .map(|num| num.wrapping_mul(*num))
        .reduce(|| 0, |a: i64, b: i64| a.wrapping_add(b));
    let finish = Instant::now() - start;
    println!(
        "Summing squares with Rayon took: {}s, sum: {}",
        finish.as_secs_f64(),
        sum
    );
}
