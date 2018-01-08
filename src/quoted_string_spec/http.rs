use lut::{Table};
use lookup_tables::{
    MediaTypeChars,
    QTextWs,
    HttpToken
};
use quoted_string::error::CoreError;
use quoted_string::spec::{
    PartialCodePoint,
    ParsingImpl,
    State,
    WithoutQuotingValidator,
};


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct HttpObsParsingImpl;

impl ParsingImpl for HttpObsParsingImpl {

    fn can_be_quoted(bch: PartialCodePoint) -> bool {
        let idx = bch.as_u8() as usize;
        idx > 0x7f || MediaTypeChars::check_at(idx, QTextWs)
    }
    fn handle_normal_state(bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
        let idx = bch.as_u8() as usize;
        if idx > 0x7f || MediaTypeChars::check_at(idx, QTextWs) {
            Ok((State::Normal, true))
        } else {
            Err(CoreError::InvalidChar)
        }
    }
}

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