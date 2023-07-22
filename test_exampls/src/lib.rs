pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn print_and_returns(a: i32) -> i32 {
    println!("I got the value {a}");
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {}

    #[test]
    fn test_printoutput() {
        let value = print_and_returns(5);
        assert_eq!(5, value);
    }

    // #[test]
    // fn test_fails() {
    //     let value = print_and_returns(5);
    //     assert_eq!(6, value);
    // }
}
