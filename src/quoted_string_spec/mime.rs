use lut::{Table, Any};
use lookup_tables::{
    MediaTypeChars,
    QText,
    QTextWs,
    DQuoteOrEscape, Ws,
    Token
};
use quoted_string::error::CoreError;
use quoted_string::spec::{
    PartialCodePoint,
    ParsingImpl,
    State,
    WithoutQuotingValidator,
    QuotingClassifier, QuotingClass,
};

use super::{MimeParsingExt, FWSState};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct MimeTokenValidator;

impl MimeTokenValidator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl WithoutQuotingValidator for MimeTokenValidator {
    fn next(&mut self, pcp: PartialCodePoint) -> bool {
        MediaTypeChars::check_at(pcp.as_u8() as usize, Token)
    }
    fn end(&self) -> bool {
        true
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct MimeObsQuoting;

impl QuotingClassifier for MimeObsQuoting {
    fn classify_for_quoting(pcp: PartialCodePoint) -> QuotingClass {
        let iu8 = pcp.as_u8();
        if MediaTypeChars::check_at(iu8 as usize, QTextWs) {
            QuotingClass::QText
        } else if iu8 <= 0x7f {
            QuotingClass::NeedsQuoting
        } else {
            QuotingClass::Invalid
        }
    }
}


macro_rules! def_mime_parsing {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            utf8 = $utf8:tt;
            obsolte_syntax = $obs:tt;
        }
        fn can_be_quoted($nm:ident: PartialCodePoint) -> bool
            $body:block
    ) => (
        $(#[$meta])*
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub struct $name(FWSState);
        impl MimeParsingExt for $name {
            const ALLOW_UTF8: bool = $utf8;
            const OBS: bool = $obs;

            fn custom_state(state: FWSState, emit: bool) -> (State<Self>, bool) {
                (State::Custom($name(state)), emit)
            }
        }

        impl ParsingImpl for $name {
            fn can_be_quoted($nm: PartialCodePoint) -> bool {
                $body
            }

            fn handle_normal_state(bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
                <Self as MimeParsingExt>::handle_normal_state(bch)
            }

            fn advance(&self, bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
                self.0.advance(bch)
            }
        }
    );
}

def_mime_parsing! {
    pub struct MimeObsParsing {
        utf8 = false;
        obsolte_syntax = true;
    }
    fn can_be_quoted(bch: PartialCodePoint) -> bool {
        // obs syntax allows any us-ascii in quoted-pairs
        bch.as_u8() <= 0x7f
    }
}

def_mime_parsing! {
    pub struct MimeObsParsingUtf8 {
        utf8 = true;
        obsolte_syntax = true;
    }
    fn can_be_quoted(bch: PartialCodePoint) -> bool {
        // Internationalized Mail does not extend quoted-pairs just qtext ...
        // obs syntax allows any us-ascii in quoted-pairs
        bch.as_u8() <= 0x7f
    }
}

def_mime_parsing! {
    pub struct MimeParsing {
        utf8 = false;
        obsolte_syntax = false;
    }
    fn can_be_quoted(bch: PartialCodePoint) -> bool {
        // VCHAR / WS == QText + Ws + DQuoteOrEscape
        let idx = bch.as_u8() as usize;
        MediaTypeChars::check_at(idx, Any::new(Ws) | QText | DQuoteOrEscape)
    }
}

def_mime_parsing! {
    pub struct MimeParsingUtf8 {
        utf8 = true;
        obsolte_syntax = false;
    }
    fn can_be_quoted(bch: PartialCodePoint) -> bool {
        // Internationalized Mail does not extend quoted-pairs just qtext ...
        let idx = bch.as_u8() as usize;
        MediaTypeChars::check_at(idx, Any::new(Ws) | QText | DQuoteOrEscape)
    }
}

