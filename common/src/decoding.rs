//! Implement what components may be parsed from the wire

use bytes::{Buf, BytesMut};
use enum_dispatch::enum_dispatch;

use crate::actions::{Abort, Continue, Discard, Quit, QuitNc, Reject, Replycode, Skip, Tempfail};

use crate::{
    error::STAGE_DECODING, AddHeader, AddRecipient, ChangeHeader, DeleteRecipient, InsertHeader,
    InvalidData, NotEnoughData, ProtocolError, Quarantine, ReplaceBody,
};

use super::commands::Connect;
use super::commands::Helo;
use super::commands::Macro;
use super::commands::Recipient;
use super::commands::Unknown;
use super::commands::{Body, EndOfBody};
use super::commands::{Data, Mail};
use super::commands::{EndOfHeader, Header};
use super::optneg::OptNeg;

/// Parse something 'from the wire'.
pub(crate) trait Parsable: Sized {
    /// The unique id code for this item
    const CODE: u8;

    /// Parse a `Self` from the given `BytesMut` buffer.
    ///
    /// # Errors
    /// This can fail to parse, returning a [`ProtocolError`].
    fn parse(buffer: BytesMut) -> Result<Self, ProtocolError>;
}

macro_rules! parse_command {
    ($container_name:ident, $($variant:ident),+$(,)?) => {
        /// See the contained variants for more.
        #[allow(missing_docs)]
        #[enum_dispatch]
        #[derive(Debug, Clone)]
        pub enum $container_name {
            $($variant($variant),)+
        }

        impl $container_name {
            /// Parse a bytes buffer into this structured data
            /// 
            /// # Errors
            /// This fn may return errors if the received data did not match
            /// valid data for this command.
            pub fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
                if buffer.is_empty() {
                    return Err(NotEnoughData::new(
                        STAGE_DECODING,
                        "Command",
                        "code missing to detect which command it is",
                        1,
                        0,
                        buffer
                    ).into());
                }
                let code = buffer.get_u8();
                match code {
                    $($variant::CODE => Ok($variant::parse(buffer)?.into()),)+
                    _ => {
                        Err(InvalidData{msg: "Unknown command sent with code", offending_bytes: BytesMut::from_iter(&[code])}.into())
                    }
                }
            }
        }

        $(impl From<$variant> for $container_name {
            fn from(value: $variant) -> Self {
                Self::$variant(value)
            }
        })+

    }
}

// Parse a command sent by the client.
parse_command!(
    // The name of this enum
    ClientCommand,
    // Include actions
    Abort,
    // Milter Control
    OptNeg,
    Quit,
    QuitNc,
    // Special Info
    Macro,
    Unknown,
    // SMTP opening
    Connect,
    Helo,
    // Header
    Mail,
    Recipient,
    Header,
    EndOfHeader,
    // Body
    Data,
    Body,
    EndOfBody,
);

// Parse a command sent by the server.
parse_command!(
    // The name of this enum
    ServerCommand,
    // Option negotiation
    OptNeg,
    // The actions
    Abort,
    Continue,
    Discard,
    Reject,
    Tempfail,
    Skip,
    Replycode,
    // Modifications
    AddRecipient,
    DeleteRecipient,
    ReplaceBody,
    AddHeader,
    InsertHeader,
    ChangeHeader,
    Quarantine,
);

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn test_create_abort() {
        let data = vec![b'A'];

        let command =
            ClientCommand::parse(BytesMut::from_iter(data)).expect("Failed parsing abort data");

        assert_matches!(command, ClientCommand::Abort(_));
    }

    #[test]
    fn test_create_optneg() {
        let data = vec![b'O', 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0];

        let command =
            ClientCommand::parse(BytesMut::from_iter(data)).expect("Failed parsing optneg data");

        assert_matches!(command, ClientCommand::OptNeg(o) if o.version == 6);
    }
}
