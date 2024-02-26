use asynchronous_codec::Decoder;
use bytes::BytesMut;
use miltr_common::{decoding::ClientCommand, ProtocolError};

use crate::codec::MilterCodec;

pub fn fuzz_parse(buffer: &mut BytesMut) -> Result<Option<ClientCommand>, ProtocolError> {
    let mut codec = MilterCodec::new(2_usize.pow(16));
    (&mut codec).decode(buffer)
}
