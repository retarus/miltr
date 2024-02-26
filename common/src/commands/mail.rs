use std::borrow::Cow;

use bytes::{BufMut, BytesMut};

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::{InvalidData, ProtocolError};
use miltr_utils::ByteParsing;

/// Information about a mail to be processed
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Mail {
    sender: BytesMut,
    esmtp_args: Option<BytesMut>,
}

impl From<&[u8]> for Mail {
    fn from(value: &[u8]) -> Self {
        Self {
            sender: BytesMut::from_iter(value),
            esmtp_args: None,
        }
    }
}

impl Mail {
    const CODE: u8 = b'M';
    /// The sender of this email
    #[must_use]
    pub fn sender(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.sender)
    }

    /// Optionally set additional esmtp args.
    ///
    /// If those are empty, an empty vector is returned.
    #[must_use]
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

impl Parsable for Mail {
    const CODE: u8 = Self::CODE;

    fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
        let Some(sender) = buffer.delimited(0) else {
            return Err(InvalidData::new(
                "Null-byte missing in mail package to sender hostname",
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

        Ok(Self { sender, esmtp_args })
    }
}

impl Writable for Mail {
    fn write(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.sender);
        buffer.put_u8(0);
        if let Some(b) = &self.esmtp_args {
            buffer.extend_from_slice(b);
        }
    }

    fn len(&self) -> usize {
        self.sender.len()
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
        self.sender.is_empty() && self.esmtp_args.is_some()
    }
}

/// SMTP Data command has been sent
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Data;

impl Data {
    const CODE: u8 = b'T';
}

impl Parsable for Data {
    const CODE: u8 = Self::CODE;

    fn parse(_buffer: BytesMut) -> Result<Self, ProtocolError> {
        Ok(Self)
    }
}

impl Writable for Data {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::decoding::Parsable;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(BytesMut::from("sender\0arg1\0arg2"), Ok( Mail {sender: BytesMut::from("sender"), esmtp_args: Some(BytesMut::from("arg1\0arg2"))}))]
    #[case(
        BytesMut::from("senderarg1arg2"),
        Err(InvalidData::new(
            "Null-byte missing in mail package to sender hostname",
            BytesMut::new(),
        ))
    )]
    fn test_mail(#[case] input: BytesMut, #[case] expected: Result<Mail, InvalidData>) {
        let parsed_mail = Mail::parse(input);

        match parsed_mail {
            Ok(mail) => {
                let expected_mail = expected.unwrap();
                assert_eq!(mail.sender, expected_mail.sender);
                assert_eq!(mail.esmtp_args, expected_mail.esmtp_args);

                //test function mail.esmtp_args()
                let vec: Vec<Cow<str>> = vec![
                    String::from_utf8_lossy(b"arg1"),
                    String::from_utf8_lossy(b"arg2"),
                ];
                assert_eq!(mail.esmtp_args(), vec);
            }

            Err(ProtocolError::InvalidData(e)) => {
                assert_eq!(e.msg, expected.unwrap_err().msg);
            }
            _ => panic!("Wrong error received"),
        }
    }

    #[cfg(feature = "count-allocations")]
    #[test]
    fn test_parse_mail() {
        use super::Mail;

        let buffer = BytesMut::from("sender\0arg1\0arg2");
        let info = allocation_counter::measure(|| {
            let res = Mail::parse(buffer);
            allocation_counter::opt_out(|| {
                println!("{:?}", res);
                assert!(res.is_ok());
            });
        });

        println!("{}", &info.count_total);
        assert_eq!(info.count_total, 1);
    }
}
