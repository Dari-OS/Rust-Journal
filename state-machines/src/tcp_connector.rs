#![allow(unused)]

use std::{
    io::{self, Read, Write},
    marker::PhantomData,
    net::{TcpStream, ToSocketAddrs},
};

pub struct Connected;
pub struct Disconnected;
pub struct NotConnected;

pub struct TcpConnection<State = NotConnected> {
    stream: Option<TcpStream>,
    state: PhantomData<State>,
}

impl TcpConnection<NotConnected> {
    pub fn new() -> Self {
        Self {
            stream: None,
            state: PhantomData,
        }
    }

    pub fn set_host<T: ToSocketAddrs>(
        self,
        host: T,
    ) -> Result<TcpConnection<Connected>, io::Error> {
        Ok(TcpConnection {
            stream: Some(TcpStream::connect(host)?),
            state: PhantomData::<Connected>,
        })
    }

    pub fn connect<T: ToSocketAddrs>(host: T) -> Result<TcpConnection<Connected>, io::Error> {
        Ok(TcpConnection {
            stream: Some(TcpStream::connect(host)?),
            state: PhantomData::<Connected>,
        })
    }
}

impl TcpConnection<Connected> {
    pub fn send(&mut self, data_to_send: &[u8]) -> io::Result<usize> {
        self.stream.as_mut().unwrap().write(data_to_send)
    }

    pub fn receive(&mut self, data_to_read: &mut [u8]) -> io::Result<usize> {
        self.stream.as_mut().unwrap().read(data_to_read)
    }

    pub fn close(self) -> TcpConnection<Disconnected> {
        if self.stream.is_some() {
            drop(self.stream.unwrap());
        }
        TcpConnection {
            stream: None,
            state: PhantomData,
        }
    }
}

impl TcpConnection<Disconnected> {
    pub fn reconnect<T: ToSocketAddrs>(
        self,
        host: T,
    ) -> Result<TcpConnection<Connected>, io::Error> {
        Ok(TcpConnection {
            stream: Some(TcpStream::connect(host)?),
            state: PhantomData::<Connected>,
        })
    }
}
