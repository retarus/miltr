use crate::decoding::Parsable;
use crate::error::STAGE_DECODING;
use crate::{NotEnoughData, ProtocolError};
use bytes::BytesMut;
use miltr_utils::ByteParsing;

/// A macro received for the command identified by `Macro.code`.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Macro {
    /// The code of the stage this macro belongs to.
    pub code: u8,
    macros: Vec<(BytesMut, BytesMut)>,
}

impl Macro {
    /// An iterator over received macros in (key, value) format.
    pub fn macros(&self) -> impl Iterator<Item = (&[u8], &[u8])> {
        self.macros.iter().map(|(b, c)| (&b[..], &c[..]))
    }
}

impl Parsable for Macro {
    const CODE: u8 = b'D';

    fn parse(mut buffer: BytesMut) -> Result<Self, ProtocolError> {
        // Basic length check
        let Some(code) = buffer.safe_get_u8() else {
            return Err(
                NotEnoughData::new(STAGE_DECODING, "Macro", "Code missing", 1, 0, buffer).into(),
            );
        };

        let field_count = bytecount::count(&buffer, 0);
        // Decode macros
        let mut macros = Vec::with_capacity(field_count / 2);
        while !buffer.is_empty() {
            let Some(name) = buffer.delimited(0) else {
                return Err(NotEnoughData::new(
                    STAGE_DECODING,
                    "Macro",
                    "missing null byte delimiter after name",
                    1,
                    0,
                    buffer,
                )
                .into());
            };

            let Some(value) = buffer.delimited(0) else {
                return Err(NotEnoughData::new(
                    STAGE_DECODING,
                    "Macro",
                    "missing null byte delimiter after value",
                    1,
                    0,
                    buffer,
                )
                .into());
            };

            macros.push((name, value));
        }

        Ok(Self { code, macros })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("O\0\0", b'O', "", "")]
    // #[case("", None)]
    #[case("Ckey\x00value\x00", b'C', "key", "value")]
    // #[case("i\x004sdsfstwg\0", "i", "4sdsfstwg")]
    fn test_parse_ok(
        #[case] input: &str,
        #[case] code: u8,
        #[case] key: &str,
        #[case] value: &str,
    ) {
        let input = BytesMut::from(input);
        let res = Macro::parse(input).expect("Parse unsuccessful");

        assert_eq!(res.code, code);
        assert_eq!(
            res.macros,
            vec![(BytesMut::from(key), BytesMut::from(value))]
        );
    }

    #[cfg(feature = "count-allocations")]
    #[test]
    fn test_parse_mmacro() {
        use super::Macro;

        let buffer = BytesMut::from("Ckey\x00value\x00");
        let info = allocation_counter::measure(|| {
            let res = Macro::parse(buffer);
            allocation_counter::opt_out(|| {
                println!("{:?}", res);
                assert!(res.is_ok());
            });
        });
        // Verify that no memory allocations are made:
        println!("{}", &info.count_total);
        assert_eq!(info.count_total, 2);
    }
}
