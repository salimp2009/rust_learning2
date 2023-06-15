#![allow(dead_code)]

#[derive(Debug)]
pub struct Boks<T> {
    p: *mut T,
}

impl<T> Drop for Boks<T> {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.p) };
        // this will drop the T but not free the Box
        // unsafe { std::ptr::drop_in_place(self.p) }
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
        assert_eq!(unsafe { *(y.p) }, x);
        println!("Boks: {:#?}", unsafe { *(y.p) });
    }
    println!("x: {x}");
}
