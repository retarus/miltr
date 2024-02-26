bitflags::bitflags! {
    /// What this milter can do.
    ///
    /// Some sendmail docs call this an 'action'.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Capability: u32 {
        /// Add headers (SMFIR_ADDHEADER)
        const SMFIF_ADDHDRS = 0x0000_0001;
        /// Change body chunks (SMFIR_REPLBODY)
        const SMFIF_CHGBODY = 0x0000_0002;
        /// Add recipients (SMFIR_ADDRCPT)
        const SMFIF_ADDRCPT = 0x0000_0004;
        /// Remove recipients (SMFIR_DELRCPT)
        const SMFIF_DELRCPT = 0x0000_0008;
        /// Change or delete headers (SMFIR_CHGHEADER)
        const SMFIF_CHGHDRS = 0x0000_0010;
        /// Quarantine message (SMFIR_QUARANTINE)
        const SMFIF_QUARANTINE = 0x0000_0020;
        /// Change the from address
        const SMFIF_CHGFROM = 0x0000_0040;
        /// Add a recipient
        const SMFIF_ADDRCPT_PAR = 0x0000_0080;
        // SMFIF_SETSYMLIST currently not supported
        // const SMFIF_SETSYMLIST = 0x0000_0100;

    }
}

impl Default for Capability {
    /// Enables all capabilities per default
    fn default() -> Self {
        Capability::all()
    }
}

impl Capability {
    /// Merge `other` capabilities with `self`
    ///
    /// Currently no version dependent merging implemented
    #[must_use]
    pub fn merge_regarding_version(self, _version: u32, other: Self) -> Self {
        self.intersection(other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_valid() {
        let input: u32 = 0x0000_0001;

        let bitflags = Capability::from_bits(input);

        assert!(bitflags.is_some());
    }

    #[test]
    fn test_create_invalid() {
        // SMFIF_SETSYMLIST is currently not supported
        let input: u32 = 0x0000_0100;

        let bitflags = Capability::from_bits(input);

        assert!(bitflags.is_none());
    }
}
