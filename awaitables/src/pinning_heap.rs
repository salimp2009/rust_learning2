use std::{marker::PhantomPinned, pin::Pin};

use futures::Future;
// using Pinning to the heap
/*
* Pinning an !Unpin type to the heap gives our data a stable address
* so we know that the data we point to can't move after it's pinned.
* In contrast to stack pinning, we know that the data will be pinned
* for the lifetime of the object.*/
#[derive(Debug)]
pub struct Test2 {
    a: String,
    b: *const String,
    // will make our type !UnPin
    _marker: PhantomPinned,
}

impl Test2 {
    pub fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test2 {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr = &boxed.a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
        boxed
    }

    pub fn get_a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    pub fn get_b(self: Pin<&Self>) -> &String {
        assert!(
            !self.b.is_null(),
            "Test:b is called without init called first"
        );
        unsafe { &*self.b }
    }
}

pub fn execute_unpin_feature(x: impl Future<Output = ()> + Unpin) {
    println!("executing unpin feature");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let mut test1 = Test2::new("test1");
        let mut test2 = Test2::new("test2");
        println!(
            "test1; a: {}, b:{}",
            test1.as_ref().get_a(),
            test1.as_ref().get_a(),
        );
        std::mem::swap(&mut test1, &mut test2);
        // std::mem::swap(&mut test1, &mut test2);
        println!(
            "test2; a: {}, b:{}",
            test2.as_ref().get_a(),
            test2.as_ref().get_a(),
        );
        println!(
            "test1; a: {}, b:{}",
            test1.as_ref().get_a(),
            test1.as_ref().get_a(),
        );
    }
}
