use std::borrow::Cow;

use bytes::{BufMut, BytesMut};

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::{InvalidData, ProtocolError};

/// Helo information sent by the smtp client
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Helo {
    buffer: BytesMut,
}

impl From<&[u8]> for Helo {
    fn from(value: &[u8]) -> Self {
        Self {
            buffer: BytesMut::from_iter(value),
        }
    }
}

impl Helo {
    const CODE: u8 = b'H';
    /// The helo greeting sent by the client
    #[must_use]
    pub fn helo(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.buffer[..])
    }
}

impl Parsable for Helo {
    const CODE: u8 = Self::CODE;

    fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
        match buffer.last() {
            None => {
                return Err(InvalidData::new(
                    "Received empty helo package, not even null terminated",
                    buffer,
                )
                .into())
            }
            Some(&x) if x != 0 => {
                return Err(InvalidData::new(
                    "Received helo package with missing null byte termination",
                    buffer,
                )
                .into())
            }
            Some(_) => buffer.split_off(buffer.len() - 1),
        };

        Ok(Self { buffer })
    }
}

impl Writable for Helo {
    fn write(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.buffer);
        buffer.put_u8(0);
    }

    fn len(&self) -> usize {
        self.buffer.len() + 1
    }

    fn code(&self) -> u8 {
        Self::CODE
    }

    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::decoding::Parsable;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(BytesMut::from("helo\0"), Ok(Helo {buffer : BytesMut::from("helo\0")} ))]
    #[case(
        BytesMut::new(),
        Err(InvalidData::new(
            "Received empty helo package, not even null terminated",
            BytesMut::new(),
        ))
    )]
    #[case(
        BytesMut::from(" "),
        Err(InvalidData::new(
            "Received helo package with missing null byte termination",
            BytesMut::new(),
        ))
    )]
    fn test_helo(#[case] input: BytesMut, #[case] expected: Result<Helo, InvalidData>) {
        let parsed_helo = Helo::parse(input);

        match parsed_helo {
            Ok(helo) => {
                let expected_helo = expected.unwrap();
                let mut buff = expected_helo.buffer;
                let _ = buff.split_off(buff.len() - 1);
                assert_eq!(helo.buffer, buff);
            }
            Err(ProtocolError::InvalidData(e)) => {
                assert_eq!(e.msg, expected.unwrap_err().msg);
            }
            _ => panic!("Wrong error received"),
        }
    }
    #[cfg(feature = "count-allocations")]
    #[test]
    fn test_parse_helo() {
        use super::Helo;

        let buffer = BytesMut::from("helo\0");
        let info = allocation_counter::measure(|| {
            let res = Helo::parse(buffer);

            allocation_counter::opt_out(|| {
                println!("{res:?}");
                assert!(res.is_ok());
            });
        });
        assert_eq!(info.count_total, 1);

        let buffer = BytesMut::new();
        let info = allocation_counter::measure(|| {
            let res = Helo::parse(buffer);

            allocation_counter::opt_out(|| {
                println!("{res:?}");
                assert!(res.is_err());
            });
        });
        println!("{}", &info.count_total);
        assert_eq!(info.count_total, 0);
    }
}
