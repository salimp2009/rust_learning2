use std::cell::UnsafeCell;

pub struct CellM<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for CellM<T> {}

// to demonstrate ; remove it after use!!
// unsafe impl<T> Sync for CellM<T> {}

impl<T> CellM<T> {
    pub fn new(value: T) -> Self {
        CellM {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, val: T) {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `Cell` is `!Sync` so this won't happen.
        // unsafe { *self.value.get() = val };
        let old = self.replace(val);
        drop(old);
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `Cell` is `!Sync` so this won't happen.
        unsafe { *self.value.get() }
    }
    pub fn replace(&self, value: T) -> T {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `Cell` is `!Sync` so this won't happen.
        std::mem::replace(unsafe { &mut *self.value.get() }, value)
    }
}

#[cfg(test)]
mod tests {
    use super::CellM;

    #[test]
    fn bad() {
        // let x = CellM::new(vec![43]);
        // let y = &x.get().[0];
        // x.set(vec![]);
    }
    #[test]
    fn bad2() {
        let x = CellM::new(3);
        let y = &x.get();
        x.set(6);
        println!("y:{y}");
        println!("x:{}", x.get());
    }

    // this was to show Cell can not be used in multithreads
    // #[test]
    // fn more_bad() {
    //     use std::sync::Arc;
    //     let x = Arc::new(CellM::new([0; 40240]));
    //     let x1 = Arc::clone(&x);
    //     let jh1 = std::thread::spawn(move || {
    //         x1.set([1; 40240]);
    //     });

    //     let x2 = Arc::clone(&x);
    //     let jh2 = std::thread::spawn(move || {
    //         x2.set([2; 40240]);
    //     });
    //     jh1.join().unwrap();
    //     jh2.join().unwrap();

    //     let xs = x.get();
    //     xs.iter().for_each(|&i| println!("{i}"));
    // }
}
