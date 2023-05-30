#![allow(clippy::drop_ref)]
use crate::cel_m::CellM;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: CellM<usize>,
}

pub struct RcM<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> RcM<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: CellM::new(1),
        });
        RcM {
            // SAFETY: Box does not give us a null ptr
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for RcM<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is allocated
        //  when the last reference goes away
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for RcM<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let count = inner.refcount.get();
        inner.refcount.set(count + 1);
        RcM {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for RcM<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let rc = inner.refcount.get();
        if rc == 1 {
            drop(inner);
            //SAFETY: we are the only Rc left and we are being dropped
            //therefore there can not be any Rc
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // there are other Rcs don't drop the box
            inner.refcount.set(rc - 1);
        }
    }
}

fn already_escaped(input: &str) -> bool {
    true
}

// example for Cow; clone-on-write smart pointer
// it can be used Borrowed or Clone and it copies fur mutating if necessary
// Borrow uses a ref and Owned use an allocated type turn &str into String
// or a slice into Vec
fn _escape<'a>(s: &'a str) -> Cow<'a, str> {
    if already_escaped(s) {
        Cow::Borrowed(s)
    } else {
        let mut string = s.to_string();
        // do somestuff
        Cow::Owned(string)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bad() {
        let (y, x): (&String, String);
        x = String::from("hello");
        y = *RcM::new(&x);
        println!("y: {}, x:{x}", *y);
    }
}
