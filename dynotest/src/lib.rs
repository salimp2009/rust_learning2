#![crate_type = "dylib"]

use std::usize;

use serde::Deserialize;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, Deserialize)]
pub struct MyType {
    pub value: usize,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
