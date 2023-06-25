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
    data: u8,
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
        vec![self.data]
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
            self.socket.set_readable_callback(wake);
            Pollm::Pending
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn simplefuture_test1() {}
}
