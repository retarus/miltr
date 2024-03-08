#![doc = include_str!("../Readme.md")]

pub mod actions;
pub mod commands;
pub mod decoding;
pub mod encoding;
pub mod modifications;
pub mod optneg;

mod error;

use encoding::ServerMessage;

pub use error::{InvalidData, NotEnoughData, ProtocolError};

use modifications::{
    body::ReplaceBody,
    headers::{AddHeader, ChangeHeader, InsertHeader},
    quarantine::Quarantine,
    recipients::{AddRecipient, DeleteRecipient},
};
