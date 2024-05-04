//! Containing data to be taken `Action`s on.
//!
//! The milter client sends data via commands, including the data it received
//! from the smtp session.

mod body;
mod connect;
mod header;
mod helo;
mod mail;
mod mmacro;
mod recipient;
mod unknown;

use enum_dispatch::enum_dispatch;

pub use self::body::{Body, EndOfBody};
pub use self::connect::{Connect, Family};
pub use self::header::{EndOfHeader, Header};
pub use self::helo::Helo;
pub use self::mail::{Data, Mail};
pub use self::mmacro::Macro;
pub use self::recipient::Recipient;
pub use self::unknown::Unknown;

/// See the respective contents about documentation
#[allow(missing_docs)]
#[enum_dispatch]
#[cfg_attr(feature = "tracing", derive(strum::Display))]
#[derive(Debug)]
pub enum Command {
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
    // Unknown
    Unknown,
}
