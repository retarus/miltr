//! Contains anything related to option negotiation between server and client

mod capability;
mod macros;
mod protocol;

use bytes::{Buf, BytesMut};
use thiserror::Error;

use crate::decoding::Parsable;
use crate::encoding::Writable;
use crate::error::STAGE_DECODING;
use crate::{NotEnoughData, ProtocolError};

pub use capability::Capability;
pub use macros::{MacroStage, MacroStages};
pub use protocol::Protocol;

/// `SMFIC_OPTNEG`
#[derive(Clone, PartialEq, Debug)]
pub struct OptNeg {
    /// The milter protocol version this implementation speaks
    pub version: u32,
    /// Which modifications this milter may send to the client
    pub capabilities: Capability,
    /// How the client should behave using this protocol
    pub protocol: Protocol,
    /// Which macros this milter would like to get from the client
    pub macro_stages: MacroStages,
}

impl Default for OptNeg {
    fn default() -> Self {
        Self {
            version: Self::VERSION,
            capabilities: Capability::default(),
            protocol: Protocol::default(),
            macro_stages: MacroStages::default(),
        }
    }
}

/// Comparing compatibilities between different optneg pacakges may produce
/// this error. See [`OptNeg::merge_compatible`] for details.
#[derive(Debug, Error)]
pub enum CompatibilityError {
    /// Thrown if this implementation does not support a received version
    #[error("Received version {received} which is not compatible with {supported}")]
    UnsupportedVersion {
        /// The version received
        received: u32,
        /// The version supported
        supported: u32,
    },
}

impl OptNeg {
    /* VERSION: the Milter protocol version that Postfix should use. The default version is 6
       (before Postfix 2.6 the default version is 2).
    */
    /* etc/postfix/main.cf:
    # Postfix ≥ 2.6
    milter_protocol = 6
    # 2.3 ≤ Postfix ≤ 2.5
    milter_protocol = 2 */

    /* If the Postfix milter_protocol setting specifies a too low version, the libmilter library will log an error message like this:

    application name: st_optionneg[xxxxx]: 0xyy does not fulfill action requirements 0xzz
    The remedy is to increase the Postfix milter_protocol version number. See, however, the limitations section below for features that aren't supported by Postfix.

    With Postfix 2.7 and earlier, if the Postfix milter_protocol setting specifies a too high version, the libmilter library simply hangs up without logging a warning, and you see a Postfix warning message like one of the following:

    warning: milter inet:host:port: can't read packet header: Unknown error : 0
    warning: milter inet:host:port: can't read packet header: Success
    warning: milter inet:host:port: can't read SMFIC_DATA reply packet header: No such file or directory
    The remedy is to lower the Postfix milter_protocol version number. Postfix 2.8 and later will automatically turn off protocol features that the application's libmilter library does not expect. */

    const VERSION: u32 = 6;

    const DATA_SIZE: usize = 4 + 4 + 4;
    const CODE: u8 = b'O';

    /// Check whether `self` is compatible with `other`
    ///
    /// This includes comparing versions, the protocol and capabilities.
    ///
    /// # Errors
    /// This errors when discovering an incompatibility between `self` and `other`
    pub fn merge_compatible(mut self, other: &Self) -> Result<Self, CompatibilityError> {
        if self.version < other.version {
            return Err(CompatibilityError::UnsupportedVersion {
                received: other.version,
                supported: self.version,
            });
        }

        self.protocol = self
            .protocol
            .merge_regarding_version(self.version, other.protocol);

        self.capabilities = self
            .capabilities
            .merge_regarding_version(self.version, other.capabilities);

        Ok(self)
    }

    // pub fn request_macro<S: ToString>(&mut self, stage: &MacroStage, macros: &[S]) {
    //     let index: u32 = stage.clone().into();
    //     self.macro_stages[index as usize] = macros.iter().map(ToString::to_string).collect();
    // }
}

impl Parsable for OptNeg {
    const CODE: u8 = Self::CODE;

    fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
        if buffer.len() != Self::DATA_SIZE {
            return Err(NotEnoughData::new(
                STAGE_DECODING,
                "Option negotiation",
                "not enough bits",
                Self::DATA_SIZE,
                buffer.len(),
                buffer,
            )
            .into());
        }

        let mut version: [u8; 4] = [0; 4];
        version.copy_from_slice(&buffer[0..4]);
        let version = u32::from_be_bytes(version);

        let mut capabilities: [u8; 4] = [0; 4];
        capabilities.copy_from_slice(&buffer[4..8]);
        let capabilities: Capability =
            Capability::from_bits_retain(u32::from_be_bytes(capabilities));

        let mut protocol: [u8; 4] = [0; 4];
        protocol.copy_from_slice(&buffer[8..12]);
        let protocol: Protocol = Protocol::from_bits_retain(u32::from_be_bytes(protocol));

        buffer.advance(12);
        Ok(Self {
            version,
            capabilities,
            protocol,
            // todo actually parse incoming macros
            macro_stages: MacroStages::default(),
        })
    }
}

//const MACRO_TEST: &[u8] = b"\x00\x00\x00\x01j {client_ptr}\x00\x00\x00\x00\x03j {rcpt_addr}\x00";

impl Writable for OptNeg {
    fn write(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.version.to_be_bytes());
        buffer.extend_from_slice(&self.capabilities.bits().to_be_bytes());
        buffer.extend_from_slice(&self.protocol.bits().to_be_bytes());

        self.macro_stages.write(buffer);
    }

    fn len(&self) -> usize {
        Self::DATA_SIZE + self.macro_stages.len()
    }

    fn code(&self) -> u8 {
        Self::CODE
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    fn ver_caps_prot() -> ([u8; 4], [u8; 4], [u8; 4]) {
        let version = [0u8, 0u8, 0u8, 6u8];
        let capabilities = [0u8, 0u8, 0u8, 255u8];
        let protocol = [0u8, 0u8, 0u8, 0u8];

        (version, capabilities, protocol)
    }

    #[cfg(feature = "count-allocations")]
    fn create_optneg_from_bytes() -> (BytesMut, ([u8; 4], [u8; 4], [u8; 4])) {
        let mut buffer = BytesMut::new();

        let (version, capabilities, protocol) = ver_caps_prot();

        buffer.extend_from_slice(&version);
        buffer.extend_from_slice(&capabilities);
        buffer.extend_from_slice(&protocol);

        (buffer, (version, capabilities, protocol))
    }

    #[cfg(feature = "count-allocations")]
    #[test]
    fn test_parse_optneg() {
        use super::OptNeg;

        let (buffer, _) = create_optneg_from_bytes();

        let info = allocation_counter::measure(|| {
            let res = OptNeg::parse(buffer);
            allocation_counter::opt_out(|| {
                println!("{res:?}");
                assert!(res.is_ok());
            });
        });
        println!("{}", &info.count_total);
        assert_eq!(info.count_total, 0);
    }

    #[test]
    fn test_write_optneg() {
        // Setup expectations
        let (version, capabilities, protocol) = ver_caps_prot();
        let mut expected = Vec::new();
        expected.extend_from_slice(&version);
        expected.extend_from_slice(&capabilities);
        expected.extend_from_slice(&protocol);

        // Write a default optneg to a buffer
        let mut buffer = BytesMut::new();
        let optneg = OptNeg::default();
        optneg.write(&mut buffer);

        // Check
        assert_eq!(optneg.len(), buffer.len());
        assert_eq!(optneg.code(), b'O');
        assert_eq!(expected, buffer.to_vec());
    }
}
