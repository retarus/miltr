use std::borrow::Cow;

use bytes::{BufMut, BytesMut};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::ProtocolError;
use crate::{error::STAGE_DECODING, InvalidData, NotEnoughData};
use miltr_utils::ByteParsing;

/// A marker for the connection family
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Family {
    Unknown = b'U',
    Unix = b'L',
    Inet = b'4',
    Inet6 = b'6',
}

impl Family {
    fn parse(buffer: &[u8]) -> Result<Self, ProtocolError> {
        match Family::try_from(buffer[0]) {
            Ok(f) => Ok(f),
            Err(_) => Err(InvalidData {
                msg: "Received unknown protocol family for connection info",
                offending_bytes: BytesMut::from_iter(&[buffer[0]]),
            }
            .into()),
        }
    }
}

/// Connect information about the smtp client
#[derive(Clone, PartialEq, Debug)]
pub struct Connect {
    hostname: BytesMut,
    /// The connection type connected to the milter client
    pub family: Family,
    /// On an IP connection, the port of the connection
    pub port: Option<u16>,
    address: BytesMut,
}

impl Connect {
    const CODE: u8 = b'C';
    /// Create a new connect package
    #[must_use]
    pub fn new(hostname: &[u8], family: Family, port: Option<u16>, address: &[u8]) -> Self {
        Self {
            hostname: BytesMut::from_iter(hostname),
            family,
            port,
            address: BytesMut::from_iter(address),
        }
    }
    /// Get the received hostname as as string-like type.
    #[must_use]
    pub fn hostname(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.hostname)
    }

    /// Get the received address as a string-like type.
    ///
    /// Remember, this can contain an IP-Address or a unix socket.
    #[must_use]
    pub fn address(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.address)
    }
}

impl Parsable for Connect {
    const CODE: u8 = Self::CODE;

    fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
        let Some(hostname) = buffer.delimited(0) else {
            return Err(InvalidData::new(
                "Null-byte missing in connection package to delimit hostname",
                buffer,
            )
            .into());
        };

        let Some(family) = buffer.safe_split_to(1) else {
            return Err(NotEnoughData::new(
                STAGE_DECODING,
                "Connect",
                "Family missing",
                1,
                2,
                buffer,
            )
            .into());
        };
        let family = Family::parse(&family)?;

        let port = {
            match family {
                Family::Inet | Family::Inet6 => {
                    let Some(buf) = buffer.safe_split_to(2) else {
                        return Err(NotEnoughData::new(
                            STAGE_DECODING,
                            "Connect",
                            "Port missing",
                            2,
                            buffer.len(),
                            buffer,
                        )
                        .into());
                    };
                    let mut raw: [u8; 2] = [0; 2];
                    raw.copy_from_slice(&buf);

                    Some(u16::from_be_bytes(raw))
                }
                _ => None,
            }
        };

        let address;
        if let Some(b'\0') = buffer.last() {
            address = buffer.split_to(buffer.len() - 1);
        } else {
            address = buffer;
        }

        let connect = Connect {
            hostname,
            family,
            port,
            address,
        };

        Ok(connect)
    }
}

impl Writable for Connect {
    fn write(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.hostname);
        buffer.put_u8(0);

        buffer.put_u8(self.family.into());

        buffer.put_u16(self.port.unwrap_or_default());

        buffer.extend_from_slice(&self.address);
        buffer.put_u8(0);
    }

    fn len(&self) -> usize {
        self.hostname.len() + 1 + 1 + 2 + self.address.len() + 1
    }

    fn code(&self) -> u8 {
        Self::CODE
    }

    fn is_empty(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Family;
    use crate::{commands::Connect, decoding::Parsable};
    use bytes::BytesMut;
    use pretty_assertions::assert_eq;

    fn initialize() -> BytesMut {
        let hostname = b"localhost";
        let family = b'4';
        let port = 1234u16.to_be_bytes();
        let address = b"127.0.0.1";

        let mut read_buffer = Vec::new();
        read_buffer.extend(hostname);
        read_buffer.push(0);
        read_buffer.push(family);
        read_buffer.extend(port);
        read_buffer.extend(address);
        read_buffer.push(0);

        BytesMut::from_iter(read_buffer)
    }

    #[tokio::test]
    async fn test_create_connect() {
        let connect = Connect::parse(initialize()).expect("Failed parsing connect");

        assert_eq!(b"localhost", connect.hostname.to_vec().as_slice());
        assert_eq!(Family::Inet, connect.family);
        assert_eq!(Some(1234), connect.port);
        assert_eq!(b"127.0.0.1", connect.address.to_vec().as_slice());
    }

    #[cfg(feature = "count-allocations")]
    #[test]
    fn test_parse_connect() {
        let buffer = initialize();

        let info = allocation_counter::measure(|| {
            let res = Connect::parse(buffer);
            allocation_counter::opt_out(|| {
                println!("{:?}", res);
                assert!(res.is_ok());
            });
        });

        println!("{}", &info.count_total);
        //4 allocations
        assert_eq!(info.count_total, 1);
    }
}
