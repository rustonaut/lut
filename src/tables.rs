
#[cfg(feature="media-type-chars")]
new_table! {
    pub flags {
        /// ObsNoWsCtl
        ObsNoWsCtl = NC,
        /// Rfc7230Token
        Rfc7230Token = HT,
        /// Token
        Token = TO,
        /// Restricted Token
        RestrictedToken = RT,
        /// AText
        AText = AT,
        /// Dtext
        DText = DT,
        /// CText
        CText = CT,
        /// QTextWs
        QText = QC
    }
    pub struct MediaTypeChars {
        static data: [u8; 128] = [
            //0x00 + 0/4/8/C
            -,                    NC,                   NC,                   NC,
            NC,                   NC,                   NC,                   NC,
            NC,                   CT|DT|QC,             -,                    NC,
            NC,                   -,                    NC,                   NC,
            //0x10  + 0/4/8/C
            NC,                   NC,                   NC,                   NC,
            NC,                   NC,                   NC,                   NC,
            NC,                   NC,                   NC,                   NC,
            NC,                   NC,                   NC,                   NC,
            //0x20 + 0/4/8/C
            CT|DT|QC,             CT|DT|AT|QC|RT|TO|HT, CT|DT,                CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|TO|HT,    CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|TO|HT,
            DT|QC,                DT|QC,                CT|DT|AT|QC|TO|HT,    CT|DT|AT|QC|RT|TO|HT,
            CT|DT|QC,             CT|DT|AT|QC|RT|TO|HT, CT|DT|QC|RT|TO|HT,    CT|DT|AT|QC,
            //0x30+ 0/4/8/C
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|QC,             CT|DT|QC,
            CT|DT|QC,             CT|DT|AT|QC,          CT|DT|QC,             CT|DT|AT|QC,
            //0x40+ 0/4/8/C
            CT|DT|QC,             CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            //0x50 + 0/4/8/C
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|QC,
            -,                    CT|QC,                CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            //0x60 + 0/4/8/C
            CT|DT|AT|QC|TO|HT,    CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            //0x70 + 0/4/8/C
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT,
            CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|RT|TO|HT, CT|DT|AT|QC|TO,
            CT|DT|AT|QC|TO|HT,    CT|DT|AT|QC|TO,       CT|DT|AT|QC|TO|HT,    NC
            //0x80
        ];
    }
}