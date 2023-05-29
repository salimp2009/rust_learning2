use crate::cel_m::CellM;

struct RcInner<T> {
    value: T,
    refcount: CellM<usize>,
}

pub struct RcM<T> {
    inner: *const RcInner<T>,
}

impl<T> RcM<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: CellM::new(1),
        });
        RcM {
            inner: Box::into_raw(inner),
        }
    }
}

// impl<T> Copy for RcM<T> {}

impl<T> Clone for RcM<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { &*self.inner };
        let count = inner.refcount.get();
        inner.refcount.set(count + 1);
        RcM { inner: self.inner }
    }
}

impl<T> std::ops::Deref for RcM<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is allocated
        //  when the last reference goes away
        &unsafe { &*self.inner }.value
    }
}

impl<T> Drop for RcM<T> {
    fn drop(&mut self) {
        let inner = unsafe { &*self.inner };
        let rc = inner.refcount.get();
        if rc == 1 {
            //SAFETY: we are the
        }
    }
}
