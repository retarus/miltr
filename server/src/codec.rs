use asynchronous_codec::{Decoder, Encoder};
use bytes::{Buf, BufMut, BytesMut};

use miltr_common::decoding::ClientCommand;
use miltr_common::encoding::ServerMessage;
use miltr_common::encoding::Writable;
use miltr_common::ProtocolError;

/// The `MilterCodec` is responsible for decoding from and encoding to bits on
/// the wire from structs provided by this crate.
///
/// It encodes behaviour about the de/encoding.
#[derive(Debug, Clone)]
pub(crate) struct MilterCodec {
    max_buffer_size: usize,
}

impl MilterCodec {
    pub(crate) fn new(max_buffer_size: usize) -> Self {
        Self { max_buffer_size }
    }
}

impl Decoder for &mut MilterCodec {
    type Item = ClientCommand;
    type Error = ProtocolError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            // Not enough data to read length marker.

            return Ok(None);
        }

        // Read length marker.
        let mut length_bytes = [0u8; 4];
        length_bytes.copy_from_slice(&src[..4]);
        let length = u32::from_be_bytes(length_bytes) as usize;

        // Check that the length is not too large to avoid a denial of
        // service attack where the server runs out of memory.
        if length > self.max_buffer_size {
            return Err(ProtocolError::TooMuchData(length));
        }

        // If arrived data is smaller than 4 bytes of length marker + the
        // decoded length, we need more data.
        if src.len() < 4 + length {
            src.reserve(4 + length - src.len());
            return Ok(None);
        }

        // Use advance to modify src such that it no longer contains
        // this frame.
        let mut parse_buf = src.split_to(4 + length);
        parse_buf.advance(4);

        Ok(Some(ClientCommand::parse(parse_buf)?))
    }
}

impl Encoder for &mut MilterCodec {
    type Item<'i> = &'i ServerMessage;
    type Error = ProtocolError;

    fn encode(&mut self, item: &ServerMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // Don't send a string if it is longer than the other end will
        // accept or  larger than we will be able to compute.
        let item_len = item.len();
        if item_len > self.max_buffer_size || item_len > usize::MAX - 1 {
            return Err(ProtocolError::TooMuchData(item_len));
        }

        let packet_len = 1_usize // single character code
            .checked_add(item_len) // The rest of the stuff
            .ok_or(ProtocolError::TooMuchData(item_len))?;

        // Convert the length into a byte array.
        // The cast to u32 cannot overflow due to the length check above.
        let packet_len_be = u32::to_be_bytes(packet_len as u32);

        // Reserve space in the buffer.
        dst.reserve(packet_len);

        // Write the length, code and string to the buffer.
        dst.extend_from_slice(&packet_len_be);
        dst.put_u8(item.code());
        item.write(dst);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_fuzz_1() {
        let input = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, b'f', b'f', 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let mut codec = MilterCodec::new(2_usize.pow(16));

        let mut buffer = BytesMut::from_iter(&input);
        let _res = (&mut codec).decode(&mut buffer);
    }

    #[test]
    fn test_decode_fuzz_2() {
        // Misssing family byte in connect package
        let input = vec![0, 0, 0, 5, 67, 58, 255, 1, 0];

        let mut codec = MilterCodec::new(2_usize.pow(16));

        let mut buffer = BytesMut::from_iter(&input);
        let _res = (&mut codec).decode(&mut buffer);
    }

    #[test]
    fn test_decode_fuzz_3() {
        // Misssing family byte in connect package
        let input = vec![
            0, 0, 0, 21, 67, 230, 186, 186, 186, 186, 42, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 186, 0, 52, 72, 255,
        ];

        let mut codec = MilterCodec::new(2_usize.pow(16));

        let mut buffer = BytesMut::from_iter(&input);
        let _res = (&mut codec).decode(&mut buffer);
    }
}
