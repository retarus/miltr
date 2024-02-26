//! Implement what components may write to the wire

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
