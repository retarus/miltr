use std::mem::size_of;

use bytes::{Buf, BytesMut};

pub trait ByteParsing {
    fn delimited(&mut self, delimiter: u8) -> Option<BytesMut>;
    fn safe_split_to(&mut self, at: usize) -> Option<BytesMut>;
    fn safe_split_off(&mut self, at: usize) -> Option<BytesMut>;
    fn safe_get_u8(&mut self) -> Option<u8>;
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
