pub fn add<T: std::ops::Add<Output = T>>(left: T, right: T) -> T {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
