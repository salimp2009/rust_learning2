extern crate num_traits;
use num_traits::ops::wrapping::WrappingAdd;

pub fn add<T: WrappingAdd<Output = T>>(left: T, right: T) -> T {
    left.wrapping_add(&right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

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
