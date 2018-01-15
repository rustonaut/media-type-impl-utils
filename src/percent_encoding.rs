use penc::EncodeSet;

use lookup_tables::{MediaTypeChars, Token, HttpToken};
use lut::Table;

/// a percent-encoding EncodeSet for Http tokens
///
/// (usable for parameter value percent encoding rfc8187)
#[derive(Copy, Clone, Debug)]
pub struct HttpPercentEncodeSet;

impl EncodeSet for HttpPercentEncodeSet {
    fn contains(&self, byte: u8) -> bool {
        //true == it needs encoding
        !MediaTypeChars::check_at(byte as usize, HttpToken)
    }
}

/// a percent-encoding EncodeSet for Mime tokens
///
/// (usable for parameter value percent encoding rfc2231)
#[derive(Copy, Clone, Debug)]
pub struct MimePercentEncodeSet;

impl EncodeSet for MimePercentEncodeSet {
    fn contains(&self, byte: u8) -> bool {
        //true == it needs encoding
        !MediaTypeChars::check_at(byte as usize, Token)
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;
    use penc::percent_encode;
    use super::{HttpPercentEncodeSet, MimePercentEncodeSet};

    #[test]
    fn what_to_encode_and_what_not_is_not_switched_around() {
        let input = "a\0b";
        let res: Cow<str> = percent_encode(input.as_bytes(), HttpPercentEncodeSet).into();
        assert_eq!(&*res, "a%00b");
        let res: Cow<str> = percent_encode(input.as_bytes(), MimePercentEncodeSet).into();
        assert_eq!(&*res, "a%00b");

    }
}