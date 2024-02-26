//! Add or delete recipients

use std::borrow::Cow;

use bytes::{BufMut, BytesMut};

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::{InvalidData, ProtocolError};
use miltr_utils::ByteParsing;

#[derive(Debug, Clone)]

///Does not change To in Header
pub struct AddRecipient {
    recipient: BytesMut,
}

impl AddRecipient {
    const CODE: u8 = b'+';

    /// Add the specified recipient
    #[must_use]
    pub fn new(recipient: &[u8]) -> Self {
        Self {
            recipient: BytesMut::from_iter(recipient),
        }
    }

    /// The recipient to add
    #[must_use]
    pub fn recipient(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.recipient)
    }
}

impl Parsable for AddRecipient {
    const CODE: u8 = Self::CODE;

    fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
        let Some(recipient) = buffer.delimited(0) else {
            return Err(InvalidData::new(
                "Received add recipient package without null byte terminating it",
                buffer,
            )
            .into());
        };

        Ok(Self { recipient })
    }
}

impl Writable for AddRecipient {
    ///buffer = recipients
    fn write(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.recipient);
        buffer.put_u8(0);
    }

    fn len(&self) -> usize {
        self.recipient.len() + 1
    }

    fn code(&self) -> u8 {
        Self::CODE
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone)]
/// Does not change To in Header
pub struct DeleteRecipient {
    recipient: BytesMut,
}

impl DeleteRecipient {
    const CODE: u8 = b'-';

    /// Delete the specified recipient
    #[must_use]
    pub fn new(recipient: &[u8]) -> Self {
        Self {
            recipient: BytesMut::from_iter(recipient),
        }
    }

    /// The (exact) recipient to be deleted
    #[must_use]
    pub fn recipient(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.recipient)
    }
}

impl Parsable for DeleteRecipient {
    const CODE: u8 = Self::CODE;

    fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
        let Some(recipient) = buffer.delimited(0) else {
            return Err(InvalidData::new(
                "Received delete recipient package without null byte terminating it",
                buffer,
            )
            .into());
        };

        Ok(Self { recipient })
    }
}

impl Writable for DeleteRecipient {
    fn write(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.recipient);
        buffer.put_u8(0);
    }

    fn len(&self) -> usize {
        self.recipient.len() + 1
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
    fn test_add_recipient() {
        let mut buffer = BytesMut::new();
        let add_rcpt = AddRecipient {
            recipient: BytesMut::from("alex@gmail"),
        };
        add_rcpt.write(&mut buffer);

        assert_eq!(buffer.len(), add_rcpt.len());
        assert_eq!(buffer, BytesMut::from("alex@gmail\0"));
    }

    #[test]
    fn test_delete_recipient() {
        let mut buffer = BytesMut::new();
        let add_rcpt = AddRecipient {
            recipient: BytesMut::from("alex@gmail"),
        };
        add_rcpt.write(&mut buffer);

        assert_eq!(buffer.len(), add_rcpt.len());
        assert_eq!(buffer, BytesMut::from("alex@gmail\0"));
    }
}
