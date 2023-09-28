use std::{slice, usize};

///  # Safety
///  raw pointer does not change the value of original reference
pub unsafe fn arbitrary_memory() {
    let address = 0x14;
    let r = address as *const i32;

    println!(" address = {:?}", address);
    println!(" pointer to address r : {:?}", r);
}

pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    if mid >= values.len() {
        return (values, &mut []);
    }
    let len = values.len();
    let ptr = values.as_mut_ptr();
    // unsafe { (&mut (*ptr)[..mid], &mut (*ptr)[mid..]) }
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let mut r2 = &mut num as *mut i32;
    unsafe {
        *r2 = 6;
        r2 = r1 as *mut i32;
    }
    println!("num: {}", num);
    unsafe {
        println!("r1: {:?}", *r1);
        println!("r2: {:?}", *r2);
    }
    unsafe {
        *r2 = 7;
    }
    println!("after 2nd mutation num: {}", num);
    unsafe {
        println!("r1: {:?}", *r1);
        println!("r2: {:?}", *r2);
    }
    // unsafe { arbitrary_memory() };
}
