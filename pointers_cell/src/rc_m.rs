#![allow(clippy::drop_ref)]
use std::ptr::NonNull;

use crate::cel_m::CellM;

struct RcInner<T> {
    value: T,
    refcount: CellM<usize>,
}

pub struct RcM<T> {
    inner: NonNull<RcInner<T>>,
}

impl<T> RcM<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: CellM::new(1),
        });
        RcM {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
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
        RcM { inner: self.inner }
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
