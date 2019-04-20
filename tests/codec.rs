use bytes::{BytesMut, BufMut};
use sonr_extras::codecs::LineCodec;
use sonr_extras::codecs::Codec;

#[test]
fn test_line_codec_encode() {
    let mut payload = BytesMut::from(&b"hello"[..]);
    let mut newline = LineCodec::new();

    let result = newline.encode(&mut payload).unwrap();
    assert_eq!(result, "hello\n");
}

#[test]
fn test_line_codec_decode() {
    let mut payload = BytesMut::from(&b"hello\n"[..]);
    let mut newline = LineCodec::new();

    let result = newline.decode(&mut payload).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_line_codec_partial_decodes() {
    let mut newline = LineCodec::new();
    let mut payload = BytesMut::from(&b"he"[..]);
    assert!(newline.decode(&mut payload).is_none());

    payload.put_slice(&b"llo\n"[..]);
    let hello = newline.decode(&mut payload).unwrap();

    assert_eq!(hello, "hello");
}

#[test]
fn test_line_codec_partial_decodes_2() {
    let mut newline = LineCodec::new();
    let mut payload = BytesMut::from(&b"he"[..]);
    assert!(newline.decode(&mut payload).is_none());

    payload.put_slice(&b"llo\nwor"[..]);
    let hello = newline.decode(&mut payload).unwrap();

    payload.put_slice(&b"ld\n"[..]);
    let world = newline.decode(&mut payload).unwrap();

    assert_eq!(hello, "hello");
    assert_eq!(world, "world");
}
