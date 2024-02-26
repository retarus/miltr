use std::io;

use bytes::BytesMut;
use thiserror::Error;

use super::optneg::CompatibilityError;

/// Encapsulating error for the different de-/encoding problems
#[derive(Debug, Error)]
pub enum ProtocolError {
    /// Data that could not be interpreted
    #[error(transparent)]
    InvalidData(#[from] InvalidData),
    /// Clearly not enough data was present
    #[error(transparent)]
    NotEnoughData(#[from] NotEnoughData),
    /// If we have a protocol compatibility issue
    #[error(transparent)]
    CompatibilityError(#[from] CompatibilityError),
    /// To much data was received to make sense
    #[error("Received a packet too large to decode (len {0})")]
    TooMuchData(usize),
    /// An io error from the underlying codec implementation
    #[error(transparent)]
    CodecError(#[from] io::Error),
}

/// Error when receiving bogus data from the other end
#[derive(Debug, Error)]
#[error("{msg}")]
pub struct InvalidData {
    /// A human readable message
    pub msg: &'static str,
    /// The data that was invalid
    pub offending_bytes: BytesMut,
}

impl InvalidData {
    /// Create a new `InvalidData` error
    #[must_use]
    pub fn new(msg: &'static str, offending_bytes: BytesMut) -> Self {
        Self {
            msg,
            offending_bytes,
        }
    }
}

pub const STAGE_DECODING: &str = "decoding";
// pub const STAGE_ENCODING: &str = "encoding";

/// Raised when definitely more data is necessary
#[derive(Debug, Error)]
#[error("{stage} {item}: expected '{expected}' bytes but got only '{got}': {msg}")]
pub struct NotEnoughData {
    /// The stage at which we are missing data
    pub stage: &'static str,
    /// The item that is missing data to wrok
    pub item: &'static str,
    /// Human readable message
    pub msg: &'static str,
    /// How many bytes where expected
    pub expected: usize,
    /// How many bytes where available
    pub got: usize,
    /// The problematic bytes
    pub buffer: BytesMut,
}

impl NotEnoughData {
    /// Create a new `NotEnoughData` error
    #[must_use]
    pub fn new(
        stage: &'static str,
        item: &'static str,
        msg: &'static str,
        expected: usize,
        got: usize,
        buffer: BytesMut,
    ) -> Self {
        Self {
            stage,
            item,
            msg,
            expected,
            got,
            buffer,
        }
    }
}
