use std::net::{TcpListener,ToSocketAddrs};

pub struct Frontend<'a, T: 'a + ToSocketAddrs> {
    addr: &'a T,
    listener: Option<TcpListener>,
}

impl<'a, T: 'a + ToSocketAddrs> Frontend<'a, T> {
    pub fn new(addr: &'a T) -> Frontend<T> {
        Frontend {
            addr: addr,
            listener: None,
        }
    }
}

