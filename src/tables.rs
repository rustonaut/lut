//! This module contains a number of lookup tables.
//!
//! Any of this lookup tables can be enabled by a feature. The additive property of
//! rust features has the effect, that if multiple crates require (a semver compatible
//! version of) lut the compiler will include only one instance of lut into the binary
//! with all features enabled which are enabled in any of the crates. As such any
//! lookup table will should only appear one time in the resulting binary.
//!
//! The benefit of having a collection of "generalish" lookup tables in on crate is that:
//!
//! 1. crates can share the same tables, even if they e.g. implement different parser algorithm
//!    decresing developer overhead and code size (if they are used in the same binary)
//! 2. developers have a place where they can look for them improving discoverability

/// a lookup table for parsing/validating Media Types (also know as MIME-Types)
///
/// To use lookup the table compile the crate with the `media-type-chars` feature enabled,
/// else wise this module will be empty.
pub mod media_type_chars {
    #[cfg(feature = "media-type-chars")]
    new_table! {

        pub flags {
            //NOTE: CText _has to_ be the first flag getting index 1
            /// CText
            CText = CT,
            /// ObsNoWsCtl
            ObsNoWsCtl = NC,
            /// Rfc7230Token
            Rfc7230Token = HT,
            /// Token
            Token = TO,
            /// Restricted Token
            RestrictedToken = RT,
            /// QText
            QText = QC,
            /// OtherObsQP  obs-quoted-pair chars not in other classes (\0, \r, \n)
            OtherObsQP = OOQP,
            /// Ws  (\t and \r)
            Ws = Ws
        }

        ///
        pub struct MediaTypeChars {
            static data: [u8; 256] = [
                //0x00 + 0/4/8/C
                OOQP,            NC,              NC,              NC,
                NC,              NC,              NC,              NC,
                NC,              Ws,              OOQP,            NC,
                NC,              OOQP,            NC,              NC,
                //0x10  + 0/4/8/C
                NC,              NC,              NC,              NC,
                NC,              NC,              NC,              NC,
                NC,              NC,              NC,              NC,
                NC,              NC,              NC,              NC,
                //0x20 + 0/4/8/C
                Ws,              CT|QC|RT|TO|HT,  CT,              CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|TO|HT,     CT|QC|RT|TO|HT,  CT|QC|TO|HT,
                QC,              QC,              CT|QC|TO|HT,     CT|QC|RT|TO|HT,
                CT|QC,           CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC,
                //0x30+ 0/4/8/C
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC,           CT|QC,
                CT|QC,           CT|QC,           CT|QC,           CT|QC,
                //0x40+ 0/4/8/C
                CT|QC,           CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                //0x50 + 0/4/8/C
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC,
                -,/*'\\'*/       CT|QC,           CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                //0x60 + 0/4/8/C
                CT|QC|TO|HT,     CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                //0x70 + 0/4/8/C
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
                CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,  CT|QC|TO,
                CT|QC|TO|HT,     CT|QC|TO,        CT|QC|TO|HT,     NC,
                //0x80
                -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -,
                //0x90
                -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -,
                //0xA0
                -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -,
                //0xB0
                -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -,
                //0xC0
                -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -,
                //0xD0
                -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -,
                //0xE0
                -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -,
                //0xF0
                -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -
            ];
        }
    }

    /// given a char is us-ascii true is returned lookup_result passed in came from looking up '"' or '\\'
    ///
    /// If a lookup result from a non-us-ascii character is passed in this function will always
    /// return true.
    ///
    /// # Implementation Details
    ///
    /// It works by taking advantage that:
    ///
    /// 1. CText is defined as the first flag having the mask repr 0b0000001
    /// 2. '"' is the only char which is only allowed in CText
    /// 3. '\\' is the only _us-ascii_ char whichs lookup result is 0
    ///
    /// As such if the char is known to be us-ascci and the lookup result is <=1 it is either
    /// '"' or '\\'
    #[cfg(feature = "media-type-chars")]
    #[inline(always)]
    pub fn is_dquote_or_escape_given_ascii(lookup_result: u8) -> bool {
        lookup_result <= 1
    }

    #[cfg(feature = "media-type-chars")]
    #[cfg(test)]
    mod test {
        use ::{Flag, Table};
        use super::{is_dquote_or_escape_given_ascii, CText, MediaTypeChars};

        #[test]
        fn ctext_has_mask_1() {
            assert_eq!(<CText as Flag<MediaTypeChars>>::BIT_MASK, 0b0000001);
        }

        #[test]
        fn only_escape_has_lookup_result_0_in_us_ascii() {
            for x in 0..128 {
                if x != '\\' as usize {
                    assert_ne!(MediaTypeChars::lookup(x), 0)
                }
            }
        }

        #[test]
        fn only_dequotes_have_lookup_result_1() {
            for x in 0..128 {
                if x != '"' as usize {
                    assert_ne!(MediaTypeChars::lookup(x), 1)
                }
            }
        }

        #[test]
        fn escape_dequotes_lt_1() {
            for x in 0..128 {
                let res = MediaTypeChars::lookup(x);
                if x == '"' as usize || x == '\\' as usize {
                    assert!(is_dquote_or_escape_given_ascii(res));
                } else {
                    assert!(!is_dquote_or_escape_given_ascii(res));
                }
            }
        }
    }
}