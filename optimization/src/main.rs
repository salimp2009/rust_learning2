#![cfg_attr(feature = "simd", feature(portable_simd))]

#[cfg(feature = "simd")]
use std::simd::Simd;

use std::{iter::zip, time::Instant, u64};

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

pub fn copy_frm_slice_test() {
    let mut vec1 = vec![1, 2, 3];
    let now = Instant::now();
    vec1.copy_from_slice(&[4, 5, 6]);
    println!("Copy from slice took: {}", now.elapsed().as_secs_f32());

    let mut vec1 = vec![1, 2, 3];
    let now = Instant::now();
    for i in 4..6 {
        vec1.push(i);
    }
    println!("Copy using push took: {}", now.elapsed().as_secs_f32());

    let mut vec1 = vec![1, 2, 3];
    let now = Instant::now();
    (4..6).for_each(|i| {
        vec1.push(i);
    });
    println!("Copy using for_each : {}", now.elapsed().as_secs_f32());
}

pub fn copy_frm_slice_big_vec() {
    let big_vec_src = vec![0; 10_000_000];
    let mut big_vec_dest = Vec::<i32>::with_capacity(10_000_000);
    let now = Instant::now();
    big_vec_src.into_iter().for_each(|val| {
        big_vec_dest.push(val);
    });
    println!("Copy using into_iter : {}", now.elapsed().as_secs_f32());

    let big_vec_src = vec![0; 10_000_000];
    let mut big_vec_dest = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec_dest.copy_from_slice(&big_vec_src);
    println!("Using copy_from_slice : {}", now.elapsed().as_secs_f32());
}

pub fn initialize() -> ([u64; 64], [u64; 64]) {
    let mut a = [0u64; 64];
    let mut b = [0u64; 64];

    (0..64).for_each(|val| {
        a[val] = u64::try_from(val).unwrap();
        b[val] = u64::try_from(val).unwrap();
    });
    (a, b)
}

#[cfg(feature = "simd")]
pub fn using_simd() {
    let (mut a, b) = initialize();

    let now = Instant::now();
    for _ in 0..100_000 {
        let c = zip(a, b).map(|(l, r)| l * r);
        let d = zip(a, c.clone()).map(|(l, r)| l + r);
        let e = zip(c, d.clone()).map(|(l, r)| l * r);
        let a = zip(e, d).map(|(l, r)| l ^ r).collect::<Vec<_>>();
    }
    println!("Without SIMD took {}s", now.elapsed().as_secs_f32());

    let (a_vec, b_vec) = initialize();
    let mut a_vec = Simd::from(a_vec);
    let b_vec = Simd::from(b_vec);

    let now = Instant::now();
    for _ in 0..100_000 {
        let c_vec = a_vec * b_vec;
        let d_vec = a_vec + c_vec;
        let e_vec = c_vec * d_vec;
        a_vec = e_vec ^ d_vec;
    }
    println!("With    SIMD took {}s", now.elapsed().as_secs_f32());
    let a_vec = a_vec.as_array();
    println!("a vec after SIMD {:#?}", a_vec[4]);
}

pub fn main() {
    // vector_alloc_size();

    // let myval = "10".to_owned();
    // is_string(&myval);
    // println!("myval; is_string ? {}", myval);
    // big_vec_test();
    // copy_frm_slice_test();
    // copy_frm_slice_big_vec();
    #[cfg(feature = "simd")]
    using_simd();
}
