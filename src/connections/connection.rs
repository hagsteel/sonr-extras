use bytes::{BufMut, Bytes, BytesMut};
use sonr::net::stream::{Stream, StreamRef};
use sonr::reactor::{Reaction, Reactor};
use std::io::{ErrorKind, Read, Result, Write};

use crate::codecs::Codec;

pub struct Connection<T: StreamRef, C: Codec> {
    stream: T,
    codec: C,
    read_buffer: BytesMut,
    write_buffer: BytesMut,
    read_capacity: usize,
    is_encoding: bool,
}

impl<T: StreamRef, C: Codec> Connection<T, C> {
    pub fn new(stream: T, codec: C, read_capacity: usize, write_capacity: usize) -> Self {
        Self {
            stream,
            codec,
            read_buffer: BytesMut::with_capacity(read_capacity),
            write_buffer: BytesMut::with_capacity(write_capacity),
            read_capacity,
            is_encoding: false,
        }
    }

    pub fn recv(&mut self) -> Option<Result<Bytes>> {
        if self.is_encoding {
            match self.codec.decode(&mut self.read_buffer) {
                Some(bytes) => return  Some(Ok(bytes)),
                None => self.is_encoding = false,
            }
        }

        if !self.stream.stream_ref().readable() {
            return None;
        }

        let res = {
            self.read_buffer.reserve(self.read_capacity);
            let mut b = unsafe { self.read_buffer.bytes_mut() };
            self.stream.stream_mut().read(&mut b)
        };

        match res {
            Ok(0) => Some(Err(ErrorKind::ConnectionReset.into())),
            Ok(n) => {
                let buf_len = self.read_buffer.len() + n;
                unsafe {
                    self.read_buffer.set_len(buf_len);
                }

                match self.codec.decode(&mut self.read_buffer) {
                    Some(bytes) => {
                        self.is_encoding = true;
                        Some(Ok(bytes))
                    }
                    None => None,
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => None,
            Err(e) => Some(Err(e)),
        }
    }

    pub fn write(&mut self) -> Option<Result<usize>> {
        if !self.stream.stream_ref().writable() {
            return None;
        }

        if self.write_buffer.is_empty() {
            return None;
        }

        match self.stream.stream_mut().write(&self.write_buffer) {
            Ok(n) => {
                self.write_buffer.split_to(n); // Remove sent data
                Some(Ok(n))
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => None,
            Err(e) => Some(Err(e)),
        }
    }

    pub fn encode(&mut self, mut payload: BytesMut) -> Bytes {
        self.codec.encode(&mut payload).unwrap()
    }

    pub fn add_payload(&mut self, payload: Bytes) {
        if payload.len() > self.write_buffer.remaining_mut() {
            self.write_buffer.reserve(payload.len());
        }
        self.write_buffer.put_slice(&payload);
    }

    pub fn react(&mut self, reaction: Reaction<()>) -> Reaction<()> {
        self.stream.stream_mut().react(reaction)
    }
}

impl<T: StreamRef, C: Codec> StreamRef for Connection<T, C> {
    type Evented = T::Evented;

    fn stream_ref(&self) -> &Stream<Self::Evented> {
        self.stream.stream_ref()
    }

    fn stream_mut(&mut self) -> &mut Stream<Self::Evented> {
        self.stream.stream_mut()
    }
}
