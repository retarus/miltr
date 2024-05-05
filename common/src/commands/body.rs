use bytes::BytesMut;

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::ProtocolError;

/// An email body part received by the milter client
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Body {
    body: BytesMut,
}

impl From<Body> for Vec<u8> {
    fn from(value: Body) -> Self {
        value.body.to_vec()
    }
}

impl From<&[u8]> for Body {
    fn from(value: &[u8]) -> Self {
        Self {
            body: BytesMut::from_iter(value),
        }
    }
}

impl Body {
    const CODE: u8 = b'B';

    /// Access the contained body bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.body
    }

    /// Access the contained body bytes mutably.
    #[must_use]
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.body
    }

    /// Convert this body to a `Vec<u8>`
    #[must_use]
    pub fn to_vec(self) -> Vec<u8> {
        self.into()
    }
}

impl Parsable for Body {
    const CODE: u8 = Self::CODE;

    fn parse(buffer: BytesMut) -> Result<Self, ProtocolError> {
        Ok(Self { body: buffer })
    }
}

impl Writable for Body {
    fn write(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.body);
    }

    fn len(&self) -> usize {
        self.body.len()
    }

    fn code(&self) -> u8 {
        Self::CODE
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
}

/// No more body parts will be received after this
#[derive(Clone, PartialEq, Debug, Default)]
pub struct EndOfBody;

impl EndOfBody {
    const CODE: u8 = b'E';
}

impl Parsable for EndOfBody {
    const CODE: u8 = Self::CODE;

    fn parse(_buffer: BytesMut) -> Result<Self, ProtocolError> {
        Ok(Self)
    }
}

impl Writable for EndOfBody {
    fn write(&self, _buffer: &mut BytesMut) {}

    fn len(&self) -> usize {
        0
    }

    fn code(&self) -> u8 {
        Self::CODE
    }

    fn is_empty(&self) -> bool {
        false
    }
}

#[cfg(all(test, feature = "count-allocations"))]
mod test {
    use super::*;

    #[test]
    fn test_parse_body() {
        let buffer = BytesMut::from("Random body...");
        let info = allocation_counter::measure(|| {
            let res = Body::parse(buffer);
            allocation_counter::opt_out(|| {
                println!("{res:?}");
                assert!(res.is_ok());
            });
        });
        // Verify that no memory allocations are made:
        assert_eq!(info.count_total, 0);
    }
}
