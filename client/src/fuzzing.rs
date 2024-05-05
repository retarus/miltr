use asynchronous_codec::Decoder;
use bytes::BytesMut;
use miltr_common::{decoding::ServerCommand, ProtocolError};

use crate::codec::MilterCodec;

pub fn fuzz_parse(buffer: &mut BytesMut) -> Result<Option<ServerCommand>, ProtocolError> {
    let mut codec = MilterCodec::new(2_usize.pow(16));
    codec.decode(buffer)
}
