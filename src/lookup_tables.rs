
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

    /// MediaTypeChars is a lookup table for a number of characterclasses relevant when parsing media types
    ///
    /// This are mainly:
    /// CText, ObsNoWsCtl, HttpToken, Token, RestrictedToken,
    /// QText, DQuotesOrEscape and Ws
    ///
    /// The classes HttpToken, Token and RestrictedToken are needed for the different specifications
    /// of a "token" in Http, Mime and for IANA regestry compatible tokens.
    ///
    /// The class CText is needed for Mime as Media-Types in Mime can contain comments :=(
    /// The class ObsNoWsCtl is needed to support the obs-part of the grammar in Mime, the
    /// obs-part in Http is different and do not need a lookup as it "just" includes any higher
    /// byte (>0x7f).
    ///
    /// Some of the classes like Ws or DQutesOrEscape are so small that they make no sense when
    /// used for themself, but they do make sense if they are combined with others or used on
    /// with lookup result already aviable.
    ///
    /// Additionally by combining aboves character classes following classes can be created by
    /// combining them, which is as fast when used for lookup:
    /// ObsQText, QTextWs, ObsQTextWs, VChar, VCharWs
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
    /// QText or Ws
    pub QTextWs = QText | Ws }
accessor_any!{
    /// QText incl. obs-parts
    pub ObsQText = QText | ObsNoWsCtl }
accessor_any!{
    /// QText incl. obs-parts or Ws
    pub ObsQTextWs = QText | ObsNoWsCtl }
accessor_any!{
    /// VChar
    pub VChar = QText | DQuoteOrEscape }
accessor_any!{
    /// VChar or Ws
    pub VCharWs = QText | DQuoteOrEscape | Ws }

