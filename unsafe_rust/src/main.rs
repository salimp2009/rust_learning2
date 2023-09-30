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

pub fn unsafe_fuction_method() {
    let mut val = vec![1, 2, 4, 5, 6];

    let (a, b) = val.split_at_mut(3);
    println!("a: {:?}, b: {:?}", a, b);

    let (aa, bb) = split_at_mut(&mut val, 3);
    println!("aa: {:?}, bb: {:?}", aa, bb);
}

pub fn ffi_functions() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("absulute value of -3 in C {}", abs(-3));
    }
}

pub fn union_access_unsafe() {
    union IntorFloat {
        i: u32,
        f: f32,
    }

    let mut u = IntorFloat { f: 1.0 };
    assert_eq!(unsafe { u.i }, 1065353216);

    u.i = 2;
    println!("u.f: {}", unsafe { u.f });
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

pub fn mutable_static_variable() {
    static mut COUNTER: u32 = 0;
    let add_to_count = |inc: u32| unsafe { COUNTER += inc };
    add_to_count(2);
    unsafe {
        println!("COUNTER {}", COUNTER);
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
    unsafe_fuction_method();
    ffi_functions();
    mutable_static_variable();
    mutable_static_variable();
    union_access_unsafe();
}
