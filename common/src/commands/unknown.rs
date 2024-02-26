use bytes::{BufMut, BytesMut};

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::{InvalidData, ProtocolError};
use miltr_utils::ByteParsing;

/// An unknown SMTP command.
///
///
/// This allows extending the SMTP protocol by special commands.
#[derive(Clone, PartialEq, Debug)]
pub struct Unknown {
    data: BytesMut,
}

impl Unknown {
    const CODE: u8 = b'U';
}

impl From<&[u8]> for Unknown {
    fn from(value: &[u8]) -> Self {
        Self {
            data: BytesMut::from(value),
        }
    }
}

impl From<BytesMut> for Unknown {
    fn from(data: BytesMut) -> Self {
        Self { data }
    }
}

impl Unknown {
    /// Access the contained body bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Access the contained body bytes mutably.
    #[must_use]
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl Writable for Unknown {
    fn write(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.data);
        buffer.put_u8(0);
    }

    fn len(&self) -> usize {
        1 + self.data.len()
    }

    fn code(&self) -> u8 {
        Self::CODE
    }

    fn is_empty(&self) -> bool {
        false
    }
}

impl Parsable for Unknown {
    const CODE: u8 = Self::CODE;

    fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
        let Some(data) = buffer.delimited(0) else {
            return Err(
                InvalidData::new("Received unknown package terminating null byte", buffer).into(),
            );
        };
        Ok(data.into())
    }
}

#[cfg(all(test, feature = "count-allocations"))]
mod test {
    use super::*;

    #[test]
    fn test_parse_unknown() {
        let buffer = BytesMut::from_iter([255, 0, 0, 0]);
        let info = allocation_counter::measure(|| {
            let _ = Unknown::parse(buffer);
        });
        // Verify memory allocations
        assert_eq!(info.count_total, 1);
    }
}
