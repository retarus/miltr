//! Carefully put this mail in a box and leave it
use std::borrow::Cow;

use bytes::{BufMut, BytesMut};

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::ProtocolError;

/// This quarantines the message into a holding pool defined by the MTA.
/// (First implemented in Sendmail in version 8.13; offered to the milter by
/// the `SMFIF_QUARANTINE` flag in "actions" of `SMFIC_OPTNEG`.)
#[derive(Debug, Clone)]
pub struct Quarantine {
    /// Give a reason to the client why this was quarantined
    reason: BytesMut,
}

impl Quarantine {
    const CODE: u8 = b'q';

    /// Quarantine with the given message
    #[must_use]
    pub fn new(reason: &[u8]) -> Self {
        Self {
            reason: BytesMut::from_iter(reason),
        }
    }

    /// Give a reason to the client why this was quarantined
    #[must_use]
    pub fn reason(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.reason)
    }
}

impl Parsable for Quarantine {
    const CODE: u8 = Self::CODE;

    fn parse(buffer: BytesMut) -> Result<Self, ProtocolError> {
        Ok(Self { reason: buffer })
    }
}

impl Writable for Quarantine {
    fn write(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.reason);
        buffer.put_u8(0);
    }

    fn len(&self) -> usize {
        self.reason.len() + 1
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
    fn test_quarantine() {
        let mut buffer = BytesMut::from("");
        let quan = Quarantine {
            reason: BytesMut::from("Invalid Input"),
        };
        quan.write(&mut buffer);

        assert_eq!(buffer, BytesMut::from("Invalid Input\0"));
    }
}
