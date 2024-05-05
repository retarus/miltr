//! Implement what components may write to the wire

#[cfg(feature = "tracing")]
use std::fmt::{self, Display};

use bytes::BytesMut;
use enum_dispatch::enum_dispatch;

use super::actions::{
    Abort, Action, Continue, Discard, Quit, QuitNc, Reject, Replycode, Skip, Tempfail,
};
use super::modifications::ModificationAction;

use super::commands::{
    Body, Command, Connect, Data, EndOfBody, EndOfHeader, Header, Helo, Mail, Recipient, Unknown,
};
use super::optneg::OptNeg;

/// Write something 'to the wire'.
#[enum_dispatch(ServerMessage)]
#[enum_dispatch(ClientMessage)]
#[enum_dispatch(ModificationAction)]
#[enum_dispatch(Command)]
#[enum_dispatch(Action)]
#[enum_dispatch(OptNeg)]
pub trait Writable {
    /// Write self to the buffer
    fn write(&self, buffer: &mut BytesMut);

    /// Byte-length that would be written if [`Self::write`] is called
    fn len(&self) -> usize;

    /// The (unique) id code of this item
    fn code(&self) -> u8;

    /// Whether a call to [`Self::write`] would write something
    fn is_empty(&self) -> bool;
}

/// Messages sent by the Server
///
/// This is used to decode things sent by the server and received by the client.
#[enum_dispatch]
#[derive(Debug)]
pub enum ServerMessage {
    /// Options received from the server
    Optneg(OptNeg),
    /// Control flow actions requested to be done by the server
    Action,
    /// Modifications requested by the server to be applied to the mail
    ModificationAction,
}

#[cfg(feature = "tracing")]
impl Display for ServerMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerMessage::Optneg(_optneg) => write!(f, "Optneg"),
            ServerMessage::Action(action) => write!(f, "Action/{action}"),
            ServerMessage::ModificationAction(mod_action) => {
                write!(f, "ModificationAction/{mod_action}")
            }
        }
    }
}

/// Messages sent by the Client
///
/// This is used to decode things sent by the client and received by the server.
#[enum_dispatch]
#[derive(Debug)]
pub enum ClientMessage {
    /// Options received from the client
    Optneg(OptNeg),
    /// Control flow actions requested by the client
    Action,
    /// SMTP commands reported by the client
    Command,
}

#[cfg(feature = "tracing")]
impl Display for ClientMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientMessage::Optneg(_optneg) => write!(f, "Optneg"),
            ClientMessage::Action(action) => write!(f, "Action/{action}"),
            ClientMessage::Command(command) => write!(f, "Command/{command}"),
        }
    }
}
