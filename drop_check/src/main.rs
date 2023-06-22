#![allow(dead_code)]
use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

#[derive(Debug)]
pub struct Boks<T> {
    p: NonNull<T>,
    _t: PhantomData<T>,
}

impl<T> Drop for Boks<T> {
    fn drop(&mut self) {
        // let _x = unsafe { read::<_>(self.p as *const u8) };
        // Safety; p was constructed from a Box and has not been freed
        // as long as self is alive
        unsafe { Box::from_raw(self.p.as_mut()) };
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
        unsafe { self.p.as_ref() }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety:is valid since it is constructed from a valid T
        // via Box which creates aligned pointer and has not been freed
        // when self exist; no other mutable reference given out to p
        unsafe { self.p.as_mut() }
    }
}

impl<T> Boks<T> {
    pub fn new(x: T) -> Self {
        Boks {
            // Safety: Box never creates a null pointer
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(x))) },
            _t: PhantomData::<T>,
        }
    }
}

#[derive(Debug, Default)]
struct Touch<T: Debug>(T);

impl<T: Debug> Drop for Touch<T> {
    fn drop(&mut self) {
        println!("touch drops: {:#?}", self.0);
    }
}

pub fn empty_iter_example() {
    let mut a = 55;
    let mut it = std::iter::empty();
    {
        let mut touches = Some(Touch(&mut a));
        touches = it.next();
        println!("touches: {:#?}", touches.unwrap_or(Touch(&mut 15)));
    }
    println!("a from empty_iter_example {:?}", a);
}

pub fn boks_lifetime() {
    let zz = String::from("Boksinsie :=)");
    // Boks is invariant over lifetime
    // therefore you cannot change 'a to 'static
    // to make it works Boks has to use NonNull<T> instead of *mut T
    let mut b = Boks::new(zz.as_str());
    let b2: Boks<&'static str> = Boks::new("hihihi");
    b = b2;
    println!("b coerced into b2 : {:#?}", *b);
}

fn main() {
    let x = 42;

    {
        let y = Boks::new(&x);
        assert_eq!(**y, x);
        println!("Boks: {:#?}", *y);
    }
    println!("x: {x}");

    let mut y = 45;
    // let _x = Boks::new(&mut y);
    let _x2 = Box::new(&mut y);
    // println!("Box: {:#?}", y);

    let mut zz = 35;
    let _b = Boks::new(Touch(&mut zz));
    // let b = Box::new(Touch(&mut zz));
    // println!("touch: {:#?}", zz);
    boks_lifetime();
    empty_iter_example();
}
