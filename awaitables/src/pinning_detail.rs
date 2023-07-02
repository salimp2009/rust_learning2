use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
pub struct Test {
    a: String,
    b: *const String,
    // will make our type !UnPin
    _marker: PhantomPinned,
}

impl Test {
    pub fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    pub fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let mut test1 = Test::new("test1");
        // test1.init();
        let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
        Test::init(test1.as_mut());
        let mut test2 = Test::new("test2");
        let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
        Test::init(test2.as_mut());
        println!(
            "test1; a: {}, b:{}",
            Test::get_a(test1.as_ref()),
            Test::get_b(test1.as_ref())
        );
        println!(
            "test2; a: {}, b:{}",
            Test::get_a(test2.as_ref()),
            Test::get_b(test2.as_ref())
        );
    }

    #[test]
    fn test_2() {
        let mut test1 = Test::new("test1");
        // test1.init();
        let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
        Test::init(test1.as_mut());
        let mut test2 = Test::new("test2");
        let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
        Test::init(test2.as_mut());
        println!(
            "test1; a: {}, b:{}",
            Test::get_a(test1.as_ref()),
            Test::get_b(test1.as_ref())
        );
        // this is illegal since test1 & test2 is pinned !
        // std::mem::swap(test1.get_mut(), test2.get_mut());
        println!(
            "test2; a: {}, b:{}",
            Test::get_a(test2.as_ref()),
            Test::get_b(test2.as_ref())
        );
    }
}
