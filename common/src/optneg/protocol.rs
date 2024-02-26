use crate::commands::Command;

bitflags::bitflags! {
    /// Protocol flags configuring communications behavior
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Protocol: u32 {
        /// MTA should not send connect info
        #[doc(alias="SMFIP_NOCONNECT")]
        const NO_CONNECT = 0x0000_0001;
            /// MTA should not send HELO info
        #[doc(alias="SMFIP_NOHELO")]
        const NO_HELO = 0x0000_0002;
        /// MTA should not send MAIL info
        #[doc(alias="SMFIP_NOMAIL")]
        const NO_MAIL = 0x0000_0004;
        /// MTA should not send RCPT info
        #[doc(alias="SMFIP_NORCPT")]
        const NO_RECIPIENT = 0x0000_0008;
        /// MTA should not send body
        #[doc(alias="SMFIP_NOBODY")]
        const NO_BODY = 0x0000_0010;
        /// MTA should not send headers
        #[doc(alias="SMFIP_NOHDRS")]
        const NO_HEADER = 0x0000_0020;
        /// MTA should not send EOH
        #[doc(alias="SMFIP_NOEOH")]
        const NO_END_OF_HEADER = 0x0000_0040;
        /// No reply for headers
        #[doc(alias="SMFIP_NR_HDR")]
        const NR_HEADER = 0x0000_0080;
        /// MTA should not send unknown commands
        #[doc(alias="SMFIP_NOUNKNOWN")]
        const NO_UNKNOWN = 0x0000_0100;
        /// MTA should not send DATA
        #[doc(alias="SMFIP_NODATA")]
        const NO_DATA =    0x0000_0200;
        /// MTA understands SMFIS_SKIP
        const SMFIP_SKIP = 0x0000_0400;
        /// MTA should also send rejected RCPTs
        const SMFIP_RCPT_REJ = 0x0000_0800;
        /// No reply for connect
        #[doc(alias="SMFIP_NR_CONN")]
        const NR_CONNECT = 0x0000_1000;
        /// No reply for HELO
        #[doc(alias="SMFIP_NR_HELO")]
        const NR_HELO = 0x0000_2000;
        /// No reply for MAIL
        #[doc(alias="SMFIP_NR_MAIL")]
        const NR_MAIL = 0x0000_4000;
        /// No reply for RCPT
        #[doc(alias="SMFIP_NR_RCPT")]
        const NR_RECIPIENT = 0x0000_8000;
        /// No reply for DATA
        #[doc(alias="SMFIP_NR_DATA")]
        const NR_DATA = 0x0001_0000;
        /// No reply for UNKN
        #[doc(alias="SMFIP_NR_UNKN")]
        const NR_UNKNOWN = 0x0002_0000;
        /// No reply for eoh
        #[doc(alias="SMFIP_NR_EOH")]
        const NR_END_OF_HEADER = 0x0004_0000;
        /// No reply for body chunk
        #[doc(alias="SMFIP_NR_BODY")]
        const NR_BODY = 0x0008_0000;
        /// header value leading space
        const SMFIP_HDR_LEADSPC = 0x0010_0000;
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Self::empty()
    }
}

impl Protocol {
    /// Whether `self` indicates that this command should be sent or not
    #[must_use]
    pub fn should_skip_send(&self, command: &Command) -> bool {
        match command {
            Command::Connect(_) => self.contains(Protocol::NO_CONNECT),
            Command::Helo(_) => self.contains(Protocol::NO_HELO),
            Command::Mail(_) => self.contains(Protocol::NO_MAIL),
            Command::Recipient(_) => self.contains(Protocol::NO_RECIPIENT),
            Command::Header(_) => self.contains(Protocol::NO_HEADER),
            Command::EndOfHeader(_) => self.contains(Protocol::NO_END_OF_HEADER),
            Command::Data(_) => self.contains(Protocol::NO_DATA),
            Command::Body(_) => self.contains(Protocol::NO_BODY),
            Command::EndOfBody(_) => false,
            Command::Unknown(_) => self.contains(Protocol::NO_UNKNOWN),
        }
    }

    /// Whether `self` indicates a response should be awaited to this command
    #[must_use]
    pub fn should_skip_response(&self, command: &Command) -> bool {
        match command {
            Command::Connect(_) => self.contains(Protocol::NR_CONNECT),
            Command::Helo(_) => self.contains(Protocol::NR_HELO),
            Command::Mail(_) => self.contains(Protocol::NR_MAIL),
            Command::Recipient(_) => self.contains(Protocol::NR_RECIPIENT),
            Command::Header(_) => self.contains(Protocol::NR_HEADER),
            Command::EndOfHeader(_) => self.contains(Protocol::NR_END_OF_HEADER),
            Command::Data(_) => self.contains(Protocol::NR_DATA),
            Command::Body(_) => self.contains(Protocol::NR_BODY),
            Command::EndOfBody(_) => false,
            Command::Unknown(_) => self.contains(Protocol::NR_UNKNOWN),
        }
    }

    /// Merge `other` protocol with `self`
    ///
    /// Currently no version dependent merging implemented
    #[must_use]
    pub fn merge_regarding_version(self, _version: u32, other: Self) -> Self {
        // No version dependent merging implemented yet
        self.intersection(other)
    }
}
