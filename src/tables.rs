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
            /// CText
            CText = CT,
            /// ObsNoWsCtl
            ObsNoWsCtl = NC,
            /// Rfc7230Token
            HttpToken = HT,
            /// Token (Mime)
            Token = TO,
            /// Restricted Token
            RestrictedToken = RT,
            /// QText
            QText = QC,
            /// The characters mainly needing escaping i.e. '"' and '\\'
            DQuoteOrEscape = DOE,
            /// Ws  (\t and \r)
            Ws = Ws
        }

        ///
        pub struct MediaTypeChars {
            static data: [u8; 256] = [
                //0x00 + 0/4/8/C
                -,               NC,              NC,              NC,
                NC,              NC,              NC,              NC,
                NC,              Ws,              -,               NC,
                NC,              -,               NC,              NC,
                //0x10  + 0/4/8/C
                NC,              NC,              NC,              NC,
                NC,              NC,              NC,              NC,
                NC,              NC,              NC,              NC,
                NC,              NC,              NC,              NC,
                //0x20 + 0/4/8/C
                Ws,              CT|QC|RT|TO|HT,  CT|DOE,          CT|QC|RT|TO|HT,
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
                DOE,/*'\\'*/     CT|QC,           CT|QC|RT|TO|HT,  CT|QC|RT|TO|HT,
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
    #[cfg(feature = "media-type-chars")]
    accessor_any!{ pub QTextWs = QText | Ws }
    #[cfg(feature = "media-type-chars")]
    accessor_any!{ pub ObsQText = QText | ObsNoWsCtl }
    #[cfg(feature = "media-type-chars")]
    accessor_any!{ pub VChar = QText | DQuoteOrEscape }

}