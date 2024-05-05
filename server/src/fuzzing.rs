//! Export functions just to enable fuzzing
//!
//! This modules is feature gated behind a private flag.

use asynchronous_codec::Decoder;
use bytes::BytesMut;
use miltr_common::{decoding::ClientCommand, ProtocolError};

use crate::codec::MilterCodec;

/// Fuzzing harness to parse the milter codec decoder
///
/// # Errors
/// Transparently returns errors from the decode function
pub fn fuzz_parse(buffer: &mut BytesMut) -> Result<Option<ClientCommand>, ProtocolError> {
    let mut codec = MilterCodec::new(2_usize.pow(16));
    (&mut codec).decode(buffer)
}
