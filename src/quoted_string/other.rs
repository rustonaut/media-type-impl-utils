use lut::{Table, Access};
use lookup_tables::{
    MediaTypeChars,
    QTextWs,
    DQuoteOrEscape,
    RestrictedToken, VCharWs
};
use qs::error::CoreError;
use qs::spec::{
    PartialCodePoint,
    ParsingImpl,
    State,
    WithoutQuotingValidator,
    QuotingClassifier, QuotingClass,
};


/// a type providing a "catch-all" `ParsingImpl` impl.
///
/// Note that because it's "catch-all" it means it supports all quircses from other impl. like
/// e.g. FWS and Comments from the Mime impl
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

/// A type providing the standart `ParsingImpl` impl.
///
/// This can be use for the (modern) http grammar and the (modern, non internationalized) mime grammar.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct NormalParsingImpl;

impl ParsingImpl for NormalParsingImpl {

    /// any VChar or Ws character can be used
    fn can_be_quoted(bch: PartialCodePoint) -> bool {
        MediaTypeChars::check_at(bch.as_u8() as usize, VCharWs)
    }

    /// any QText or Ws character advances the normal state everything else is invalid
    fn handle_normal_state(bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
        if MediaTypeChars::check_at(bch.as_u8() as usize, QTextWs) {
            Ok((State::Normal, true))
        } else {
            Err(CoreError::InvalidChar)
        }
    }

}

/// A type providing a strict `ParsingImpl`.
///
/// The strict parser is based on the constraints for registering media-types with IANA.
///
/// A media type compatible with this impl should be valid in any context in which a media
/// type can appear.
///
/// All media types _should_ be compatible with this impl, but they do not have to.
///
/// Note that this impl is a bit more strict than just IANA registry compatibility wrt.
/// quoted-strings in media-types as it only allows quoted-pairs with chars which can not be
/// represented in another way i.e. `'"'` and `'\\'`.
///
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct StrictParsingImpl;

impl ParsingImpl for StrictParsingImpl {

    /// only allow `'"'` and `'\\'`
    fn can_be_quoted(bch: PartialCodePoint) -> bool {
        let iu8 = bch.as_u8();
        iu8 == b'"' || iu8 == b'\\'
    }

    /// any qtext or ws is ok, others are invalid
    fn handle_normal_state(bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
        if MediaTypeChars::check_at(bch.as_u8() as usize, QTextWs) {
            Ok((State::Normal, true))
        } else {
            Err(CoreError::InvalidChar)
        }
    }
}



/// a type providing a "catch-all" `QuotingClassifier` impl
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

/// a type providing a "catch-all" impl for `QuotingClassifier`
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

/// a variation or `NormalUtf8Quoting` treating all non-us-ascii chars as qtext
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct NormalUtf8Quoting;

impl QuotingClassifier for NormalUtf8Quoting {

    fn classify_for_quoting(pcp: PartialCodePoint) -> QuotingClass {
        let idx = pcp.as_u8() as usize;
        if idx > 0x7f {
            QuotingClass::QText
        } else {
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
}

/// a type providing a strict `WithoutQuotingValidator` impl
///
/// The restrictions are based on the restrictions applied to media types which can be registered
/// with IANA.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct StrictTokenValidator {
    count: usize
}

impl WithoutQuotingValidator for StrictTokenValidator {
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
        use qs::spec::ParsingImpl;
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
            use qs::spec::State;
            use qs::error::CoreError;
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
        use qs::spec::ParsingImpl;
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
            use qs::spec::State;
            use qs::error::CoreError;
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
        use qs::spec::QuotingClassifier;
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
        use qs::spec::QuotingClassifier;
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
            StrictTokenValidator, PartialCodePoint
        };

        mod next {
            use super::*;

            #[test]
            fn single_ascii_alphanumeric_char_is_valid() {
                let mut vali = StrictTokenValidator::default();
                assert!(vali.next(PartialCodePoint::from_code_point('A' as u32)));
                assert!(vali.end());

            }

            #[test]
            fn simple_token() {
                let text = "vnd.extra.yay+noo";
                let mut vali = StrictTokenValidator::default();
                for bch in text.bytes() {
                    assert!(vali.next(PartialCodePoint::from_utf8_byte(bch)));
                }
                assert!(vali.end());
            }

            #[test]
            fn failed_next_does_not_increase_counter() {
                let mut vali = StrictTokenValidator::default();
                assert!(!vali.next(PartialCodePoint::from_code_point('~' as u32)));
                assert_eq!(vali.count, 0);
            }
        }

        mod end {
            use super::*;



            #[test]
            fn length_limit_is_checked() {
                let mut vali = StrictTokenValidator::default();
                for _ in 0..128 {
                    assert!(vali.next(PartialCodePoint::from_code_point('a' as u32)));
                }
                assert!(!vali.end())
            }

            #[test]
            fn after_content_is_ok() {
                let mut vali = StrictTokenValidator::default();
                for _ in 0..127 {
                    assert!(vali.next(PartialCodePoint::from_code_point('a' as u32)));
                }
                assert!(vali.end())
            }

            #[test]
            fn after_content_after_failed_is_ok() {
                let mut vali = StrictTokenValidator::default();
                for _ in 0..5 {
                    assert!(vali.next(PartialCodePoint::from_code_point('a' as u32)));
                }
                assert!(!vali.next(PartialCodePoint::from_code_point('~' as u32)));
                assert!(vali.end())
            }

        }
    }
}