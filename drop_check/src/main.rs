#![allow(dead_code)]
pub struct Boks<T> {
    p: *mut T,
}

impl<T> Boks<T> {
    pub fn new(x: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(x)),
        }
    }
}

fn main() {}
