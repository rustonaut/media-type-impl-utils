
new_table! {

    pub flags {
        /// CText characters
        CText = CT,

        /// ObsNoWsCtl characters of the `obs-` part of the mime grammar which are not ws and not ctl
        /// by combining this with other classes the `obs-` part of the grammar can be supported
        /// e.g. `QText` + `ObsNoWsCtl` == `ObsQText` i.e. `QText` in the `obs-` grammar
        ObsNoWsCtl = NC,

        /// HttToken character allowed in a token given the grammar from RFC7230 (http)
        HttpToken = HT,

        /// Token (Mime) characters allowed in a token given the grammar from RFC2045 (mime)
        Token = TO,

        /// Restricted Token, character allowed in a token wrt. to registering media types at IANA,
        /// any registered media type has to be compatible with this any other should be, through
        /// there is no guarantee for it.
        RestrictedToken = RT,

        /// QText characters, i.e. characters which can appear in a quoted string
        /// without being escaped through quoted pairs
        QText = QC,

        /// The characters mainly needing escaping (just `'"'` and `'\\'`)
        ///
        /// Note: while using a lookup just for to determine if it is `'"'` or `'\\'` makes little sense
        /// using it with a already looked up value or in combination with others can make sense
        DQuoteOrEscape = DOE,

        /// Ws  (just `'\t'` and `'\r'`)
        ///
        /// Note: while using a lookup just for to determine if it is `' '` or `'\t'` makes little sense
        /// using it with a already looked up value or in combination with others can make sense
        Ws = Ws
    }

    /// MediaTypeChars is a lookup table for a number of character classes relevant when parsing media types
    ///
    /// This are mainly:
    /// `CText`, `ObsNoWsCtl`, `HttpToken`, `Token`, `RestrictedToken`,
    /// `QText`, `DQuotesOrEscape` and `Ws`
    ///
    /// The classes `HttpToken`, `Token` and `RestrictedToken` are needed for the different specifications
    /// of a "token" in Http, Mime and for IANA registry compatible tokens.
    ///
    /// The class `CText` is needed for Mime as Media-Types in Mime can contain comments :=(
    /// The class `ObsNoWsCtl` is needed to support the obs-part of the grammar in Mime, the
    /// obs-part in Http is different and do not need a lookup as it "just" includes any higher
    /// byte (>0x7f).
    ///
    /// Some of the classes like `Ws` or `DQutesOrEscape` are so small that they make no sense when
    /// used for themself, but they do make sense if they are combined with others or used on
    /// with lookup result already available.
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

accessor_any!{
    /// QText or Ws chars
    pub QTextWs = QText | Ws }
accessor_any!{
    /// QText incl. obs-parts of the mime grammar
    pub ObsQText = QText | ObsNoWsCtl }
accessor_any!{
    /// QText incl. obs-parts (mime) or ws
    pub ObsQTextWs = QText | ObsNoWsCtl }
accessor_any!{
    /// VChar printable us-ascii chars (i.e. `'!' <= ch && ch <= '~'`)
    pub VChar = QText | DQuoteOrEscape }
accessor_any!{
    /// VChar or Ws (i.e. `(' ' <= ch && ch <= '~') || ch == '\t'`)
    pub VCharWs = QText | DQuoteOrEscape | Ws }

