//! Replace body parts

use std::borrow::Cow;

use bytes::BytesMut;

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::ProtocolError;

/// Replace the body of the incoming mail.
///
/// If this modification action is used, the **whole** body has to be sent back.
/// It can be split across multiple `ReplaceBody` actions, but in the end,
/// the complete intended response has to be sent.
#[derive(Debug, Clone)]
pub struct ReplaceBody {
    body: BytesMut,
}

impl<'a> FromIterator<&'a u8> for ReplaceBody {
    fn from_iter<T: IntoIterator<Item = &'a u8>>(into_iter: T) -> Self {
        Self {
            body: into_iter.into_iter().copied().collect(), // body: BytesMut::from_iter(into_iter.into_iter().copied())
        }
    }
}

impl ReplaceBody {
    const CODE: u8 = b'b';

    /// A body part to replace the original
    #[must_use]
    pub fn new(body: &[u8]) -> Self {
        Self {
            body: BytesMut::from_iter(body),
        }
    }

    /// The body to send back.
    ///
    /// Will be interpreted by the client as a valid mail.
    #[must_use]
    pub fn body(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.body)
    }
}

impl Parsable for ReplaceBody {
    const CODE: u8 = Self::CODE;

    fn parse(buffer: BytesMut) -> Result<Self, ProtocolError> {
        Ok(Self { body: buffer })
    }
}

impl Writable for ReplaceBody {
    /// A milter that uses `SMFIR_REPLBODY` must replace the entire body
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
        self.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_replace_body() {
        let mut buffer = BytesMut::from("b");
        let replace_body = ReplaceBody {
            body: BytesMut::from("new body"),
        };
        replace_body.write(&mut buffer);

        assert_eq!(buffer, BytesMut::from("bnew body"));
    }
}
