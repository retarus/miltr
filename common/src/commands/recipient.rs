use std::borrow::Cow;

use bytes::{BufMut, BytesMut};

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::{InvalidData, ProtocolError};
use miltr_utils::ByteParsing;

/// An smtp recipient
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Recipient {
    recipient: BytesMut,
    esmtp_args: Option<BytesMut>,
}

impl From<&[u8]> for Recipient {
    fn from(value: &[u8]) -> Self {
        Self {
            recipient: BytesMut::from_iter(value),
            esmtp_args: None,
        }
    }
}

impl Recipient {
    const CODE: u8 = b'R';
    /// The recipient as received by the milter client
    #[must_use]
    pub fn recipient(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.recipient)
    }

    /// Optional esmtp arguments regarding the recipients.
    ///
    /// Returns an empty `Vec` if no esmtp args where received
    pub fn esmtp_args(&self) -> Vec<Cow<str>> {
        let Some(args) = &self.esmtp_args else {
            return Vec::new();
        };

        args[..]
            .split(|&b| b == 0)
            .map(String::from_utf8_lossy)
            .collect()
    }
}

impl Parsable for Recipient {
    const CODE: u8 = Self::CODE;

    fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
        let Some(recipient) = buffer.delimited(0) else {
            return Err(InvalidData::new(
                "Received recipient package without recipient terminated by null byte in it",
                buffer,
            )
            .into());
        };

        let esmtp_args = {
            if buffer.is_empty() {
                None
            } else {
                Some(buffer)
            }
        };

        Ok(Self {
            recipient,
            esmtp_args,
        })
    }
}

impl Writable for Recipient {
    fn write(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.recipient);
        buffer.put_u8(0);

        if let Some(b) = &self.esmtp_args {
            buffer.extend_from_slice(b);
        }
    }

    fn len(&self) -> usize {
        self.recipient.len()
            + 1
            + self
                .esmtp_args
                .as_ref()
                .map(BytesMut::len)
                .unwrap_or_default()
    }

    fn code(&self) -> u8 {
        Self::CODE
    }

    fn is_empty(&self) -> bool {
        self.recipient.is_empty() && self.esmtp_args.is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::decoding::Parsable;
    use rstest::rstest;

    #[rstest]
    #[case(BytesMut::from("recipient1 recipient2\0arg1\0arg2"), Ok( Recipient {recipient: BytesMut::from("recipient1 recipient2"), esmtp_args: Some(BytesMut::from("arg1\0arg2"))}))]
    #[case(
        BytesMut::from("recipient1 arg1 arg2"),
        Err(InvalidData::new(
            "Received recipient package without recipient terminated by null byte in it",
            BytesMut::new(),
        ))
    )]
    fn test_recipient(#[case] input: BytesMut, #[case] expected: Result<Recipient, InvalidData>) {
        let parsed_recp = Recipient::parse(input);

        match parsed_recp {
            Ok(recp) => {
                let expected_recp = expected.unwrap();
                assert_eq!(recp.recipient, expected_recp.recipient);
                assert_eq!(recp.esmtp_args, expected_recp.esmtp_args);

                //test function mail.esmtp_args()
                let vec: Vec<Cow<'_, str>> = vec![
                    String::from_utf8_lossy(b"arg1"),
                    String::from_utf8_lossy(b"arg2"),
                ];
                assert_eq!(recp.esmtp_args(), vec);
            }
            Err(ProtocolError::InvalidData(e)) => {
                assert_eq!(e.msg, expected.unwrap_err().msg);
            }
            _ => panic!("Wrong error received"),
        }
    }

    #[cfg(feature = "count-allocations")]
    #[test]
    fn test_parse_recipient() {
        use super::Recipient;

        let buffer = BytesMut::from("rcpt\0arg1\0arg2");
        let info = allocation_counter::measure(|| {
            let res = Recipient::parse(buffer);
            allocation_counter::opt_out(|| {
                println!("{res:?}");
                assert!(res.is_ok());
            });
        });
        //2 allocation
        assert!((0..2).contains(&info.count_total));
    }
}
