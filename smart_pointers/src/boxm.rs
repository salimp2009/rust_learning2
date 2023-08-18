use std::ops::Deref;

#[derive(Debug)]
pub struct Boxm<T>(T);

impl<T> Boxm<T> {
    // add code here
    pub fn new(value: T) -> Self {
        Boxm(value)
    }
}

impl<T> Deref for Boxm<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
