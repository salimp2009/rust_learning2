#![allow(dead_code)]
#![allow(unused_imports)]
use std::vec;

pub trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Pollm<Self::Output>;
}

pub enum Pollm<T> {
    Ready(T),
    Pending,
}

pub struct Socket {
    data: Vec<u8>,
}
impl Socket {
    pub fn has_data_to_read(&self) -> bool {
        println!("data ready");
        true
    }
    pub fn set_readable_callback(&self, wake: fn()) {
        wake();
    }

    pub fn read_buf(&self) -> Vec<u8> {
        self.data[..].to_vec()
    }
}
struct SocketReadm<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketReadm<'_> {
    type Output = Vec<u8>;
    fn poll(&mut self, wake: fn()) -> Pollm<Self::Output> {
        if self.socket.has_data_to_read() {
            Pollm::Ready(self.socket.read_buf())
            // Pollm::Ready(vec![1, 2, 3, 4])
        } else {
            // The socket does not yet have data.
            //
            // Arrange for `wake` to be called once data is available.
            // When data becomes available, `wake` will be called, and the
            // user of this `Future` will know to call `poll` again and
            // receive data.self.socket.set_readable_callback(wake);
            Pollm::Pending
        }
    }
}

pub struct Joinm<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Joinm<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Pollm<Self::Output> {
        if let Some(a) = &mut self.a {
            if let Pollm::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        if let Some(b) = &mut self.b {
            if let Pollm::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            Pollm::Ready(())
        } else {
            Pollm::Pending
        }
    }
}

pub struct AndThen<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThen<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Pollm<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                Pollm::Ready(()) => self.first.take(),
                Pollm::Pending => return Pollm::Pending,
            };
        }
        self.second.poll(wake)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn simplefuture_test1() {}
}
