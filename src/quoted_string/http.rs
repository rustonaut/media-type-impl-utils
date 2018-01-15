use lut::{Table};
use lookup_tables::{
    MediaTypeChars,
    QTextWs,
    HttpToken
};
use qs::error::CoreError;
use qs::spec::{
    PartialCodePoint,
    ParsingImpl,
    State,
    WithoutQuotingValidator,
};

/// a zero-sized type to provide a `ParsingImpl` for media types wrt. the (obs) Http grammar
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct HttpObsParsingImpl;

impl ParsingImpl for HttpObsParsingImpl {

    /// any qtext, ws and non-us-ascii char can be quoted
    fn can_be_quoted(bch: PartialCodePoint) -> bool {
        let idx = bch.as_u8() as usize;
        idx > 0x7f || MediaTypeChars::check_at(idx, QTextWs)
    }
    /// any qtext, ws and non-us-ascii char can appear without quoting, and
    /// all chars are semantic relevant (emit=true)
    fn handle_normal_state(bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
        let idx = bch.as_u8() as usize;
        if idx > 0x7f || MediaTypeChars::check_at(idx, QTextWs) {
            Ok((State::Normal, true))
        } else {
            Err(CoreError::InvalidChar)
        }
    }
}

/// a zero-sized type to provide a `WithoutQuotingValidator` impl for tokens (http grammar)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct HttpTokenValidator;

impl HttpTokenValidator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl WithoutQuotingValidator for HttpTokenValidator {
    fn next(&mut self, pcp: PartialCodePoint) -> bool {
        MediaTypeChars::check_at(pcp.as_u8() as usize, HttpToken)
    }

    fn end(&self) -> bool {
        true
    }
}