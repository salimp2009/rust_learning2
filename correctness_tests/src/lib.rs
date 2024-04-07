extern crate num_traits;
use num_traits::ops::wrapping::WrappingAdd;
// use std::ops::Add;

pub fn add<T: WrappingAdd<Output = T>>(left: T, right: T) -> T {
    left.wrapping_add(&right)
}

pub fn add_wrapped(left: u64, right: u64) -> u64 {
    // left + right
    left.wrapping_add(right)
}

pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    (1..n + 1).for_each(|i| {
        if i % 3 == 0 && i % 5 == 0 {
            result.push("FizzBuzz".into());
        } else if i % 3 == 0 {
            result.push("Fizz".into());
        } else if i % 5 == 0 {
            result.push("Buzz".into());
        } else {
            result.push(i.to_string());
        }
    });
    result
}

pub fn better_fizz(n: i32) -> Vec<String> {
    use std::collections::HashMap;
    let mapping = HashMap::from([(3, "Fizz"), (5, "Buzz")]);
    let mut result = vec![String::new(); n as usize];
    let mut keys: Vec<&i32> = mapping.keys().collect();
    keys.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use proptest::prelude::*;
    use std::sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    };
    static mut COUNT: AtomicI32 = AtomicI32::new(0);
    // below does not compile since static variable can only be called in
    // const functions, tuple structs and tuple variants
    // static COUNT2: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));

    lazy_static! {
        static ref COUNT2: Mutex<i32> = Mutex::new(0);
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizz_buzz(3), vec!["1", "2", "Fizz"]);
        assert_eq!(fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(
            fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz",
            ]
        )
    }
    // these count test may fail depending on the order
    // if assert_eq! are commented out;
    // since cargo test runs on multiple threads

    #[test]
    fn test_lazy_static() {
        let mut count = COUNT2.lock().expect("couldn't acquire lock");
        *count += 1;
        println!("COUNT2: first test {:?}", count);
    }

    #[test]
    fn test_lazy_static2() {
        let mut count = COUNT2.lock().expect("couldn't acquire lock");
        *count += 1;
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
        #[test]
        fn test_unsigned(a: u64, b:u64) {
            assert_eq!(add_wrapped(a, b), a.wrapping_add(b));
        }
    }
}
