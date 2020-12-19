use std::net::SocketAddr;
use std::ops::Deref;

pub struct Datagram {
    d: Vec<u8>,
    src: SocketAddr,
    dst: SocketAddr
}

impl Datagram {
    pub fn new<V: Into<Vec<u8>>>(src: SocketAddr, dst: SocketAddr, d: V) -> Self {
        Self {
            d: d.into(),
            src,
            dst
        }
    }

    pub fn source(&self) -> SocketAddr {
        self.src
    }

    pub fn destination(&self) -> SocketAddr {
        self.dst
    }
}

impl Deref for Datagram {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.d
    }
}
