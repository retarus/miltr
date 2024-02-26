use bytes::BytesMut;

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::ProtocolError;

/// Quit this connection gracefully
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Quit;

impl Quit {
    const CODE: u8 = b'Q';
}

impl Parsable for Quit {
    const CODE: u8 = Self::CODE;

    fn parse(_buffer: BytesMut) -> Result<Self, ProtocolError> {
        Ok(Self)
    }
}

impl Writable for Quit {
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

/// This one mail processing is finished, but re-use this connection for the next one.i
#[derive(Clone, PartialEq, Debug, Default)]
pub struct QuitNc;

impl QuitNc {
    const CODE: u8 = b'K';
}

impl Parsable for QuitNc {
    const CODE: u8 = Self::CODE;

    fn parse(_buffer: BytesMut) -> Result<Self, ProtocolError> {
        Ok(Self)
    }
}

impl Writable for QuitNc {
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

#[cfg(all(test, feature = "count-allocations"))]
mod test {
    use bytes::BytesMut;

    use crate::decoding::Parsable;

    #[test]
    fn test_parse_quit() {
        use super::Quit;

        let buffer = BytesMut::from("this is quit buffer...");
        let info = allocation_counter::measure(|| {
            let _ = Quit::parse(buffer);
        });
        //No allocation
        assert_eq!(info.count_total, 0);
    }
}
