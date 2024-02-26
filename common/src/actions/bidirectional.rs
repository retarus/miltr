use bytes::BytesMut;

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::ProtocolError;

/// Abort / finish processing a mail.
///
/// This Signals the other end to either:
/// - abort processing of the current mail
/// - finish up processing if at the end of a mail processing flow
#[derive(Debug, Clone)]
pub struct Abort;

impl Parsable for Abort {
    const CODE: u8 = b'A';

    fn parse(_buffer: BytesMut) -> Result<Self, ProtocolError> {
        Ok(Self)
    }
}

impl Writable for Abort {
    fn write(&self, _buffer: &mut BytesMut) {}

    fn len(&self) -> usize {
        0
    }

    fn code(&self) -> u8 {
        Self::CODE
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Continue with the next step in the milter protocol
#[derive(Debug, Clone)]
pub struct Continue;

impl Continue {
    const CODE: u8 = b'c';
}

impl Parsable for Continue {
    const CODE: u8 = Self::CODE;

    fn parse(_buffer: BytesMut) -> Result<Self, ProtocolError> {
        Ok(Self)
    }
}

impl Writable for Continue {
    fn write(&self, _buffer: &mut BytesMut) {}

    fn len(&self) -> usize {
        0
    }

    fn code(&self) -> u8 {
        Self::CODE
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
