use lut::{Table, Access};
use lookup_tables::{
    MediaTypeChars,
    QTextWs,
    DQuoteOrEscape,
    RestrictedToken, VCharWs
};
use quoted_string::error::CoreError;
use quoted_string::spec::{
    PartialCodePoint,
    ParsingImpl,
    State,
    WithoutQuotingValidator,
    QuotingClassifier, QuotingClass,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct AnyParsingImpl;

impl ParsingImpl for AnyParsingImpl {

    fn can_be_quoted(_bch: PartialCodePoint) -> bool {
        true
    }

    fn handle_normal_state(_bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
        Ok((State::Normal, true))
    }

}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct NormalParsingImpl;

impl ParsingImpl for NormalParsingImpl {

    fn can_be_quoted(bch: PartialCodePoint) -> bool {
        MediaTypeChars::check_at(bch.as_u8() as usize, VCharWs)
    }

    fn handle_normal_state(bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
        if MediaTypeChars::check_at(bch.as_u8() as usize, QTextWs) {
            Ok((State::Normal, true))
        } else {
            Err(CoreError::InvalidChar)
        }
    }

}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct StrictParsingImpl;

impl ParsingImpl for StrictParsingImpl {

    fn can_be_quoted(bch: PartialCodePoint) -> bool {
        let iu8 = bch.as_u8();
        iu8 == b'"' || iu8 == b'\\'
    }

    fn handle_normal_state(bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
        if MediaTypeChars::check_at(bch.as_u8() as usize, QTextWs) {
            Ok((State::Normal, true))
        } else {
            Err(CoreError::InvalidChar)
        }
    }
}




#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct AnyQuoting;

impl QuotingClassifier for AnyQuoting {
    fn classify_for_quoting(pcp: PartialCodePoint) -> QuotingClass {
        let iu8 = pcp.as_u8();
        if iu8 == b'"' || iu8 == b'\\' {
            QuotingClass::NeedsQuoting
        } else {
            QuotingClass::QText
        }
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct NormalQuoting;

impl QuotingClassifier for NormalQuoting {

    fn classify_for_quoting(pcp: PartialCodePoint) -> QuotingClass {
        let idx = pcp.as_u8() as usize;
        let lres = MediaTypeChars::lookup(idx);
        if QTextWs.check(lres) {
            QuotingClass::QText
        } else if DQuoteOrEscape.check(lres) && idx <= 0x7f {
            QuotingClass::NeedsQuoting
        } else {
            QuotingClass::Invalid
        }
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct RestrictedTokenValidator {
    count: usize
}

impl WithoutQuotingValidator for RestrictedTokenValidator {
    fn next(&mut self, pcp: PartialCodePoint) -> bool {
        let iu8 = pcp.as_u8();
        let res =
            if self.count == 0 {
                iu8 < 0x7f && (iu8 as char).is_alphanumeric()
            } else {
                MediaTypeChars::check_at(iu8 as usize, RestrictedToken)
            };
        if res {
            self.count += 1;
        }
        res
    }

    fn end(&self) -> bool {
        self.count < 128
    }
}