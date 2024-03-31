extern crate num_traits;
use num_traits::ops::wrapping::WrappingAdd;

pub fn add<T: WrappingAdd<Output = T>>(left: T, right: T) -> T {
    left.wrapping_add(&right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use proptest::prelude::*;
    use std::sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    };
    static mut COUNT: AtomicI32 = AtomicI32::new(0);
    // below does not compile since static variable can only be called in
    // const functions, tuple structs and tuple variants
    // static COUNT2: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));

    lazy_static! {
        static ref COUNT2: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));
    }
    // these count test may fail depending on the order
    // if assert_eq! are commented out;
    // since cargo test runs on multiple threads

    #[test]
    fn test_lazy_static() {
        let count = Arc::clone(&COUNT2);
        count.fetch_add(1, Ordering::AcqRel);
        println!("COUNT2: first test {:?}", count);
    }

    #[test]
    fn test_lazy_static2() {
        let count = Arc::clone(&COUNT2);
        count.fetch_add(1, Ordering::AcqRel);
        println!("COUNT2: second test {:?}", count);
    }

    #[test]
    fn test_count_unsafe() {
        unsafe {
            COUNT.fetch_add(1, Ordering::AcqRel);
            println!("COUNT1: test1 {:?}", COUNT);
            // assert_eq!(COUNT.load(Ordering::Acquire), 1);
        }
    }
    #[test]
    fn test_count_unsafe2() {
        unsafe {
            COUNT.fetch_add(1, Ordering::AcqRel);
            println!("COUNT1: test2 {:?}", COUNT);
            // assert_eq!(COUNT.load(Ordering::Acquire), 2);
        }
    }

    #[test]
    fn it_works() {
        let result = add::<i32>(2, 2);
        assert_eq!(result, 4);
        let a = "left".to_string();
        let b = "right";
        assert!(std::ops::Add::add(a, b) == *"leftright");
        // assert!(a + &b == "leftright".to_string());
        // add("left".to_string(), "+right");
    }
    proptest! {
        #[test]
        fn test_add(a:i64, b:i64) {
            assert_eq!(add(a, b), a.wrapping_add(b));
        }
    }
}
