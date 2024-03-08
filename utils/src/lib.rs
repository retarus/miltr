#![doc = include_str!("../Readme.md")]

use std::mem::size_of;

use bytes::{Buf, BytesMut};

/// Safe extensions to methods from [`bytes::BytesMut`].
pub trait ByteParsing {
    /// Split at the given delimiter.
    ///
    /// Return the split off bytes without the delimiter
    fn delimited(&mut self, delimiter: u8) -> Option<BytesMut>;

    /// Bounds checked variant of [`bytes::BytesMut::split_to`]
    fn safe_split_to(&mut self, at: usize) -> Option<BytesMut>;

    /// Bounds checked variant of [`bytes::BytesMut::split_off`]
    fn safe_split_off(&mut self, at: usize) -> Option<BytesMut>;

    /// Bounds checked variant of [`bytes::BytesMut::get_u8`]
    fn safe_get_u8(&mut self) -> Option<u8>;

    /// Bounds checked variant of [`bytes::BytesMut::get_u32`]
    fn safe_get_u32(&mut self) -> Option<u32>;
}

impl ByteParsing for BytesMut {
    fn delimited(&mut self, delimiter: u8) -> Option<BytesMut> {
        let index = self.iter().position(|&b| b == delimiter)?;

        let off = self.split_to(index);
        self.advance(1);

        Some(off)
    }

    fn safe_split_to(&mut self, at: usize) -> Option<Self> {
        if at > self.len() {
            return None;
        }
        Some(self.split_to(at))
    }

    fn safe_split_off(&mut self, at: usize) -> Option<Self> {
        if at > self.capacity() {
            return None;
        }
        Some(self.split_off(at))
    }

    fn safe_get_u8(&mut self) -> Option<u8> {
        if self.is_empty() {
            return None;
        }
        Some(self.get_u8())
    }

    fn safe_get_u32(&mut self) -> Option<u32> {
        if self.len() < size_of::<u32>() {
            return None;
        }
        Some(self.get_u32())
    }
}
