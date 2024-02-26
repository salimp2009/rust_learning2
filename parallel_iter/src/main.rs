use ::std::time::Instant;
use std::{char, i64};

use rand::Rng;
use rayon::prelude::*;
use regex::bytes::Regex;

pub fn generate_random_values(count: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut ret = Vec::with_capacity(count);

    (0..count).for_each(|_| ret.push(rng.gen_range(i64::MIN..i64::MAX)));
    ret
}

pub fn generate_random_strings(count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut ret = Vec::with_capacity(count);

    (0..count).for_each(|_| {
        let mut rstring = String::new();
        rstring.push(char::from_u32(rng.gen_range(0..127)).unwrap());
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
    // let data = generate_random_values(1_000);

    // let start = Instant::now();
    // let sum = data
    //     .iter()
    //     .map(|num| num.wrapping_mul(*num))
    //     .reduce(|a: i64, b: i64| a.wrapping_add(b));
    // let finish = Instant::now() - start;
    // println!(
    //     "Summing squares without Rayon took: {}s, sum: {}",
    //     finish.as_secs_f64(),
    //     sum.unwrap()
    // );
    // // using parallel iterators
    // let start = Instant::now();
    // let sum = data
    //     .par_iter()
    //     .map(|num| num.wrapping_mul(*num))
    //     .reduce(|| 0, |a: i64, b: i64| a.wrapping_add(b));
    // let finish = Instant::now() - start;
    // println!(
    //     "Summing squares with Rayon took: {}s, sum: {}",
    //     finish.as_secs_f64(),
    //     sum
    // );

    let data = generate_random_strings(500);

    // Regex is Send + Sync. Can be used in parallel filter with Rayon
    let regx = Regex::new(r"cAtdog").unwrap();
    let start = Instant::now();
    let matches: Vec<_> = data
        .iter()
        .filter(|strng| regx.is_match(strng.as_bytes()))
        .collect();
    let finish = Instant::now() - start;
    println!("Regex took {}s", finish.as_secs_f64());
    println!("Regex matches {:#?}s", matches);

    let start = Instant::now();
    let matches: Vec<_> = data
        .par_iter()
        .filter(|strng| regx.is_match(strng.as_bytes()))
        .collect();
    let finish = Instant::now() - start;
    println!("Regex with Rayon took {}s", finish.as_secs_f64());
    println!("Regex with Rayon matches {:#?}s", matches);

    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = b"I categorically deny having triskaidekaphobia.";
    assert!(re.is_match(hay));
}
