#![allow(dead_code)]

use std::{
    ops::{Deref, DerefMut},
    ptr::read,
};

#[derive(Debug)]
pub struct Boks<T> {
    p: *mut T,
}

impl<T> Drop for Boks<T> {
    fn drop(&mut self) {
        // let _x = unsafe { read::<_>(self.p as *const u8) };
        // Safety; p was constructed from a Box and has not been freed
        // as long as self is alive
        unsafe { Box::from_raw(self.p) };
        // this will drop the T but not free the Box
        // unsafe { std::ptr::drop_in_place(self.p) }
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety:is valid since it is constructed from a valid T
        // via Box which creates aligned pointer and has not been freed
        // when self exist
        unsafe { &*(self.p) }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety:is valid since it is constructed from a valid T
        // via Box which creates aligned pointer and has not been freed
        // when self exist; no other mutable reference given out to p
        unsafe { &mut *(self.p) }
    }
}

impl<T> Boks<T> {
    pub fn new(x: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(x)),
        }
    }
}

fn main() {
    let x = 42;

    {
        let y = Boks::new(x);
        assert_eq!(*y, x);
        println!("Boks: {:#?}", *y);
    }
    println!("x: {x}");

    let mut y = 45;
    let x = Boks::new(&mut y);
    // println!("Boks: {:#?}", y);
    drop(x);
}
