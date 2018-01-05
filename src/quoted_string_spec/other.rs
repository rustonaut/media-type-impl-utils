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
        } else if DQuoteOrEscape.check(lres) {
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

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    mod NormalParsingImpl {
        //it's not unused rustc just falls over wrt. the unused detection and `use super::*`
        #[allow(unused_imports)]
        use quoted_string::spec::ParsingImpl;
        use super::super::{NormalParsingImpl, PartialCodePoint};

        mod can_be_quoted {
            use super::*;

            #[test]
            fn all_vchars_and_ws_can_be_quoted() {
                for x in 0u8..0x7f {
                    let pcp = PartialCodePoint::from_utf8_byte(x);
                    let can_be_quoted = NormalParsingImpl::can_be_quoted(pcp);
                    match x {
                        b'\t' | b' '...b'~' => assert!(can_be_quoted),
                        _ => assert!(!can_be_quoted)
                    }
                }
            }
        }

        mod handle_normal_state {
            use quoted_string::spec::State;
            use quoted_string::error::CoreError;
            use super::*;

            #[test]
            fn non_ws_ctls_are_invalid() {
                for x in 1u8..31 {
                    if x == b'\t' { continue }
                    let pcp = PartialCodePoint::from_utf8_byte(x);
                    let res = NormalParsingImpl::handle_normal_state(pcp);
                    assert_eq!(res, Err(CoreError::InvalidChar))
                }
                let pcp = PartialCodePoint::from_utf8_byte(127);
                let res = NormalParsingImpl::handle_normal_state(pcp);
                assert_eq!(res, Err(CoreError::InvalidChar))
            }

            #[test]
            fn ws_is_valid() {
                for x in &[b'\t', b' '] {
                    let pcp = PartialCodePoint::from_utf8_byte(*x);
                    let res = NormalParsingImpl::handle_normal_state(pcp);
                    assert_eq!(res, Ok((State::Normal, true)))
                }
            }

            #[test]
            fn vchars_expect_dquotes_and_escape_are_valid() {
                for x in b'!'..(b'~'+1) {
                    if x == b'\\' || x == b'"' { continue; }
                    let pcp = PartialCodePoint::from_utf8_byte(x);
                    let res = NormalParsingImpl::handle_normal_state(pcp);
                    assert_eq!(res, Ok((State::Normal, true)))
                }
            }
        }
    }

    mod StrictParsingImpl {
        //it's not unused rustc just falls over wrt. the unused detection and `use super::*`
        #[allow(unused_imports)]
        use quoted_string::spec::ParsingImpl;
        use super::super::{StrictParsingImpl, PartialCodePoint};

        mod can_be_quoted {
            use super::*;

            #[test]
            fn only_dquotes_and_escape_can_be_quoted() {
                for x in &[b'"',b'\\'] {
                    let pcp = PartialCodePoint::from_utf8_byte(*x);
                    let can_be_quoted = StrictParsingImpl::can_be_quoted(pcp);
                    match *x {
                        b'"' | b'\\' => assert!(can_be_quoted),
                        _ => assert!(!can_be_quoted)
                    }
                }
            }
        }

        mod handle_normal_state {
            use quoted_string::spec::State;
            use quoted_string::error::CoreError;
            use super::*;

            #[test]
            fn non_ws_ctls_are_invalid() {
                for x in 1u8..31 {
                    if x == b'\t' { continue }
                    let pcp = PartialCodePoint::from_utf8_byte(x);
                    let res = StrictParsingImpl::handle_normal_state(pcp);
                    assert_eq!(res, Err(CoreError::InvalidChar))
                }
                let pcp = PartialCodePoint::from_utf8_byte(127);
                let res = StrictParsingImpl::handle_normal_state(pcp);
                assert_eq!(res, Err(CoreError::InvalidChar))
            }

            #[test]
            fn ws_is_valid() {
                for x in &[b'\t', b' '] {
                    let pcp = PartialCodePoint::from_utf8_byte(*x);
                    let res = StrictParsingImpl::handle_normal_state(pcp);
                    assert_eq!(res, Ok((State::Normal, true)))
                }
            }

            #[test]
            fn vchars_expect_dquotes_and_escape_are_valid() {
                for x in b'!'..(b'~'+1) {
                    if x == b'\\' || x == b'"' { continue; }
                    let pcp = PartialCodePoint::from_utf8_byte(x);
                    let res = StrictParsingImpl::handle_normal_state(pcp);
                    assert_eq!(res, Ok((State::Normal, true)))
                }
            }
        }
    }

    mod AnyQuoting {
        //it's not unused rustc just falls over wrt. the unused detection and `use super::*`
        #[allow(unused_imports)]
        use quoted_string::spec::QuotingClassifier;
        use super::super::{PartialCodePoint, AnyQuoting, QuotingClass};

        mod classify_for_quoting {
            use super::*;

            #[test]
            fn dquotes_and_escape_needs_quoting() {
                for x in &[b'"', b'\\'] {
                    let pcp = PartialCodePoint::from_utf8_byte(*x);
                    let res = AnyQuoting::classify_for_quoting(pcp);
                    assert_eq!(res, QuotingClass::NeedsQuoting);
                }
            }

            #[test]
            fn non_dquoted_escape_is_ok() {
                for x in 0..255 {
                    if x == b'"' || x == b'\\' { continue }
                    let pcp = PartialCodePoint::from_utf8_byte(x);
                    let res = AnyQuoting::classify_for_quoting(pcp);
                    assert_eq!(res, QuotingClass::QText);
                }
            }
        }

    }

    mod NormalQuoting {
        //it's not unused rustc just falls over wrt. the unused detection and `use super::*`
        #[allow(unused_imports)]
        use quoted_string::spec::QuotingClassifier;
        use super::super::{PartialCodePoint, NormalQuoting, QuotingClass};

        mod classify_for_quoting {
            use super::*;

            #[test]
            fn dquotes_and_escape_needs_quoting() {
                for x in &[b'"', b'\\'] {
                    let pcp = PartialCodePoint::from_utf8_byte(*x);
                    let res = NormalQuoting::classify_for_quoting(pcp);
                    assert_eq!(res, QuotingClass::NeedsQuoting);
                }
            }

            #[test]
            fn qtext_is_qtext() {
                for x in b'!'..(b'~'+1) {
                    if x == b'"' || x == b'\\' { continue }
                    let pcp = PartialCodePoint::from_utf8_byte(x);
                    let res = NormalQuoting::classify_for_quoting(pcp);
                    assert_eq!(res, QuotingClass::QText);
                }
            }

            #[test]
            fn ws_is_qtext() {
                for x in &[ b' ', b'\t'] {
                    let pcp = PartialCodePoint::from_utf8_byte(*x);
                    let res = NormalQuoting::classify_for_quoting(pcp);
                    assert_eq!(res, QuotingClass::QText);
                }
            }

            #[test]
            fn ctls_are_invalid() {
                for x in 0..b' ' {
                    if x == b'\t' { continue }
                    let pcp = PartialCodePoint::from_utf8_byte(x);
                    let res = NormalQuoting::classify_for_quoting(pcp);
                    assert_eq!(res, QuotingClass::Invalid, "0d{:?}", x);
                }
                let pcp = PartialCodePoint::from_utf8_byte(0x7f);
                let res = NormalQuoting::classify_for_quoting(pcp);
                assert_eq!(res, QuotingClass::Invalid);
            }
       }

    }

    mod RestrictedTokenValidator {
        //it's a rust bug not detecting it's used when doing the unused checks
        #[allow(unused_imports)]
        use super::super::WithoutQuotingValidator;
        use super::super::{
            RestrictedTokenValidator, PartialCodePoint
        };

        mod next {
            use super::*;

            #[test]
            fn single_ascii_alphanumeric_char_is_valid() {
                let mut vali = RestrictedTokenValidator::default();
                assert!(vali.next(PartialCodePoint::from_code_point('A' as u32)));
                assert!(vali.end());

            }

            #[test]
            fn simple_token() {
                let text = "vnd.extra.yay+noo";
                let mut vali = RestrictedTokenValidator::default();
                for bch in text.bytes() {
                    assert!(vali.next(PartialCodePoint::from_utf8_byte(bch)));
                }
                assert!(vali.end());
            }

            #[test]
            fn failed_next_does_not_increase_counter() {
                let mut vali = RestrictedTokenValidator::default();
                assert!(!vali.next(PartialCodePoint::from_code_point('~' as u32)));
                assert_eq!(vali.count, 0);
            }
        }

        mod end {
            use super::*;



            #[test]
            fn length_limit_is_checked() {
                let mut vali = RestrictedTokenValidator::default();
                for _ in 0..128 {
                    assert!(vali.next(PartialCodePoint::from_code_point('a' as u32)));
                }
                assert!(!vali.end())
            }

            #[test]
            fn after_content_is_ok() {
                let mut vali = RestrictedTokenValidator::default();
                for _ in 0..127 {
                    assert!(vali.next(PartialCodePoint::from_code_point('a' as u32)));
                }
                assert!(vali.end())
            }

            #[test]
            fn after_content_after_failed_is_ok() {
                let mut vali = RestrictedTokenValidator::default();
                for _ in 0..5 {
                    assert!(vali.next(PartialCodePoint::from_code_point('a' as u32)));
                }
                assert!(!vali.next(PartialCodePoint::from_code_point('~' as u32)));
                assert!(vali.end())
            }

        }
    }
}