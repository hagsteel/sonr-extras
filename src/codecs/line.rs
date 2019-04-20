use bytes::{BufMut, Bytes, BytesMut};

use super::{Codec, CodecError};

pub struct LineCodec {
    index: usize,
    offset: usize,
}

/// Encode / decode bytes separated by new line chars.
///
/// ```
/// # use bytes::BytesMut;
/// # use sonr_extras::codecs::LineCodec;
/// # use sonr_extras::codecs::Codec;
/// fn main() {
///     let mut payload = BytesMut::from(&b"hello\n"[..]);
///     let mut newline = LineCodec::new();
///
///     let result = newline.decode(&mut payload).unwrap();
///     assert_eq!(result, "hello");
/// }
/// ```
impl LineCodec {
    pub fn new() -> Self {
        Self {
            index: 0,
            offset: 0,
        }
    }
}

impl Codec for LineCodec {
    fn decode(&mut self, buf: &mut BytesMut) -> Option<Bytes> {
        match buf[self.index..].iter().position(|b| *b == b'\n') {
            Some(index) => {
                self.index += index;
                let mut bytes = buf.split_to(self.index);

                // Remove trailing \r
                if let Some(b'\r') = bytes.last() {
                    bytes.truncate(bytes.len() - 1);
                }

                let bytes = bytes.freeze();
                if buf.len() > 0 {
                    buf.advance(1);
                }
                self.index = 0;
                Some(bytes)
            }
            None => {
                self.index = buf.len();
                None
            }
        }
    }

    fn encode(&mut self, buf: &mut BytesMut) -> Result<Bytes, CodecError> {
        if buf.len() + 1 >= buf.capacity() {
            buf.reserve(1);
        }
        buf.put_u8(b'\n');
        Ok(buf.clone().freeze())
    }
}
