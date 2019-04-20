use bytes::{BufMut, Bytes, BytesMut};
use sonr::net::stream::{Stream, StreamRef};
use sonr::reactor::{Reaction, Reactor};
use std::io::{ErrorKind::WouldBlock, Read, Write};

use crate::codecs::Codec;

pub struct Connection<T: StreamRef, C: Codec> {
    stream: T,
    codec: C,
    read_buffer: BytesMut,
    write_buffer: BytesMut,
}

impl<T: StreamRef, C: Codec> Connection<T, C> {
    pub fn new(stream: T, codec: C, read_cap: usize, write_cap: usize) -> Self {
        Self {
            stream,
            codec,
            read_buffer: BytesMut::with_capacity(read_cap),
            write_buffer: BytesMut::with_capacity(write_cap),
        }
    }

    pub fn recv(&mut self) -> Option<Result<Bytes, ()>> {
        if !self.stream.stream_ref().readable() {
            return None;
        }

        let res = {
            let mut b = unsafe { self.read_buffer.bytes_mut() };
            self.stream.stream_mut().read(&mut b)
        };

        match res {
            // The connection was closed by the peer.
            Ok(0) => Some(Err(())),

            // Try to decode messages from the read data
            Ok(n) => {
                let buf_len = self.read_buffer.len() + n;
                unsafe {
                    self.read_buffer.set_len(buf_len);
                }

                match self.codec.decode(&mut self.read_buffer) {
                    Some(bytes) => Some(Ok(bytes)),
                    None => None,
                }
            }

            // Not an actual error
            Err(ref e) if e.kind() == WouldBlock => None,

            // Connection closed. Ignoring the reason
            // for simplicity
            Err(_) => Some(Err(())),
        }
    }

    pub fn write(&mut self) -> Option<Result<usize, ()>> {
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
            Err(ref e) if e.kind() == WouldBlock => None,
            Err(_) => Some(Err(())),
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

