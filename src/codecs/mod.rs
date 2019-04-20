use bytes::{BytesMut, Bytes};

pub(crate) mod line;

#[derive(Debug)]
pub enum CodecError {
    Other(String)
}

pub trait Codec {
    fn decode(&mut self, buf: &mut BytesMut) -> Option<Bytes>;
    fn encode(&mut self, buf: &mut BytesMut) -> Result<Bytes, CodecError>;
}
