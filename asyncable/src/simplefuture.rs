pub trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Pollm<Self::Output>;
}

pub enum Pollm<T> {
    Ready(T),
    Pending,
}

#[allow(dead_code)]
struct Socket {
    data: Vec<u8>,
}

#[allow(dead_code)]
impl Socket {
    fn has_data_to_read(&self) -> bool {
        !self.data.is_empty()
    }
    fn read_buf(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn set_readable_callback(&self, wake: fn()) {
        wake();
    }
}

#[allow(dead_code)]
pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Pollm<Self::Output> {
        if self.socket.has_data_to_read() {
            Pollm::Ready(self.socket.read_buf())
        } else {
            self.socket.set_readable_callback(wake);
            Pollm::Pending
        }
    }
}

#[allow(dead_code)]
pub struct Joinm<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}
impl<FutureA, FutureB> SimpleFuture for Joinm<FutureA, FutureB> {
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Pollm<Self::Output> {
        todo!()
    }
}
