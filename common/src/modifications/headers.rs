//! Add, change or insert smtp headers

use std::borrow::Cow;

use bytes::{BufMut, BytesMut};

use crate::commands::Header;
use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::error::STAGE_DECODING;
use crate::{NotEnoughData, ProtocolError};
use miltr_utils::ByteParsing;

/// Add a header
#[derive(Debug, Clone)]
pub struct AddHeader {
    header: Header,
}

impl AddHeader {
    const CODE: u8 = b'h';

    /// Create a Header from some bytes
    #[must_use]
    pub fn new(name: &[u8], value: &[u8]) -> Self {
        Self {
            header: Header::new(name, value),
        }
    }

    /// The name of the header
    #[must_use]
    pub fn name(&self) -> Cow<str> {
        self.header.name()
    }

    /// The value of the header
    #[must_use]
    pub fn value(&self) -> Cow<str> {
        self.header.value()
    }
}

impl Parsable for AddHeader {
    const CODE: u8 = Self::CODE;

    fn parse(buffer: BytesMut) -> Result<Self, ProtocolError> {
        let header = Header::parse(buffer)?;

        Ok(Self { header })
    }
}

impl Writable for AddHeader {
    fn write(&self, buffer: &mut BytesMut) {
        self.header.write(buffer);
    }

    fn len(&self) -> usize {
        self.header.len()
    }

    fn code(&self) -> u8 {
        Self::CODE
    }
    fn is_empty(&self) -> bool {
        self.header.is_empty()
    }
}

/// Change an existing header
#[derive(Debug, Clone)]
pub struct ChangeHeader {
    /// The index in a list of headers sharing `name` which to change
    ///
    /// Headers can be set multiple times. This index is only valid in the
    /// context of headers with the same name.
    index: u32,

    header: Header,
}

impl ChangeHeader {
    const CODE: u8 = b'm';

    /// Create a Header from some bytes
    #[must_use]
    pub fn new(index: u32, name: &[u8], value: &[u8]) -> Self {
        Self {
            index,
            header: Header::new(name, value),
        }
    }

    /// The name of the header
    #[must_use]
    pub fn name(&self) -> Cow<str> {
        self.header.name()
    }

    /// The value of the header
    #[must_use]
    pub fn value(&self) -> Cow<str> {
        self.header.value()
    }

    /// The index in a list of headers sharing `name` which to change
    ///
    /// Headers can be set multiple times. This index is only valid in the
    /// context of headers with the same name.
    #[must_use]
    pub fn index(&self) -> u32 {
        self.index
    }
}

impl Parsable for ChangeHeader {
    const CODE: u8 = Self::CODE;

    fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
        let Some(index) = buffer.safe_get_u32() else {
            return Err(NotEnoughData::new(
                STAGE_DECODING,
                "ChangeHeader",
                "Index byte missing",
                1,
                0,
                buffer,
            )
            .into());
        };
        let header = Header::parse(buffer)?;

        Ok(Self { index, header })
    }
}

impl Writable for ChangeHeader {
    ///index : uint32 is Index of the occurrence of this header.
    /// index has to > 0.
    ///(Note that the "index" above is per-name--i.e. a 3 in this field
    ///indicates that the modification is to be applied to the third such
    ///header matching the supplied "name" field.  A zero length string for
    ///"value", leaving only a single NUL byte, indicates that the header
    ///should be deleted entirely.)
    fn write(&self, buffer: &mut BytesMut) {
        let index = u32::to_be_bytes(self.index);
        buffer.put_slice(&index);
        self.header.write(buffer);
    }

    fn len(&self) -> usize {
        4 + self.header.len()
    }

    fn code(&self) -> u8 {
        Self::CODE
    }
    fn is_empty(&self) -> bool {
        self.header.is_empty()
    }
}

/// Insert header at a specified position (modification action)
#[derive(Debug, Clone)]
pub struct InsertHeader {
    index: u32,
    header: Header,
}

impl InsertHeader {
    const CODE: u8 = b'i';

    /// Create a Header from some bytes
    #[must_use]
    pub fn new(index: u32, name: &[u8], value: &[u8]) -> Self {
        Self {
            index,
            header: Header::new(name, value),
        }
    }

    /// The name of the header
    #[must_use]
    pub fn name(&self) -> Cow<str> {
        self.header.name()
    }

    /// The value of the header
    #[must_use]
    pub fn value(&self) -> Cow<str> {
        self.header.value()
    }

    /// The list index at which to insert this header
    #[must_use]
    pub fn index(&self) -> u32 {
        self.index
    }
}

impl Parsable for InsertHeader {
    const CODE: u8 = Self::CODE;

    fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
        let Some(index) = buffer.safe_get_u32() else {
            return Err(NotEnoughData::new(
                STAGE_DECODING,
                "InsertHeader",
                "Index byte missing",
                1,
                0,
                buffer,
            )
            .into());
        };
        let header = Header::parse(buffer)?;

        Ok(Self { index, header })
    }
}

impl Writable for InsertHeader {
    ///index is Index into header list where insertion should occur
    fn write(&self, buffer: &mut BytesMut) {
        let index = u32::to_be_bytes(self.index);
        buffer.put_slice(&index);
        self.header.write(buffer);
    }

    fn len(&self) -> usize {
        4 + self.header.len()
    }

    fn code(&self) -> u8 {
        Self::CODE
    }
    fn is_empty(&self) -> bool {
        self.header.is_empty()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_add_header() {
        let mut buffer = BytesMut::from("h");
        let add_header = AddHeader {
            header: Header::new(b"name", b"value"),
        };

        add_header.write(&mut buffer);
        assert_eq!(buffer, BytesMut::from("hname\0value\0"));
    }

    #[rstest]
    #[case((1, String::from("name"), String::from("value")), BytesMut::from("m\0\0\0\x01name\0value\0"))]
    #[case((0, String::from("name"), String::from("value")), BytesMut::from("m\0\0\0\0name\0value\0"))]
    #[case((2, String::from("name"), String::from("\0")), BytesMut::from("m\0\0\0\x02name\0\0\0"))]
    fn test_change_header(#[case] input: (u32, String, String), #[case] expected: BytesMut) {
        let mut buffer = BytesMut::from("m");

        let change_header = ChangeHeader {
            index: input.0,
            header: Header::new(input.1.as_bytes(), input.2.as_bytes()),
        };

        change_header.write(&mut buffer);

        assert_eq!(buffer, expected);
    }
    #[rstest]
    #[case((1, String::from("name"), String::from("value")), BytesMut::from("i\0\0\0\x01name\0value\0"))]
    #[case((0, String::from("name"), String::from("value")), BytesMut::from("i\0\0\0\0name\0value\0"))]
    #[case((2, String::from("name"), String::from("\0")), BytesMut::from("i\0\0\0\x02name\0\0\0"))]
    fn test_insert_header(#[case] input: (u32, String, String), #[case] expected: BytesMut) {
        let mut buffer = BytesMut::from("i");

        let change_header = ChangeHeader {
            index: input.0,
            header: Header::new(input.1.as_bytes(), input.2.as_bytes()),
        };

        change_header.write(&mut buffer);

        assert_eq!(buffer, expected);
    }
}
