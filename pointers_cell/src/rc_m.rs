use std::ops::Deref;

struct RcInner<T> {
    value: T,
    refcount: usize,
}

pub struct RcM<T> {
    inner: *const RcInner<T>,
}

// impl<T> Copy for RcM<T> {}

// impl<T> Clone for RcM<T> {
//     fn clone(&self) -> Self {
//         let inner = unsafe { &*self.inner };
//         inner.refcount += 1;
//         RcM { inner: self.inner }
//     }
// }

impl<T> std::ops::Deref for RcM<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { &*self.inner }.value
    }
}
