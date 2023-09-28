pub fn arbitrary_memory() {
    let address = 0x14;
    let r = address as *const i32;

    println!(" address = {:?}", address);
    println!(" pointer to address r : {:?}", r);
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
    // arbitrary
}
