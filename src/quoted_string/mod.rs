use lut::{Table, Access};
use lookup_tables::{
    MediaTypeChars,
    ObsQText, QText,
    ObsQTextWs, QTextWs,
    Ws
};
use qs::error::CoreError;
use qs::spec::{
    PartialCodePoint,
    ParsingImpl,
    State
};

mod other;
pub use self::other::*;
mod http;
pub use self::http::*;
mod mime;
pub use self::mime::*;

/// This is an extension trait for implementing MediaType parsing in context of Mime
pub trait MimeParsingExt: ParsingImpl {
    /// is true if utf8 is allowed
    const ALLOW_UTF8: bool;
    /// is true if the `obs-` part of the grammar is supported
    const OBS: bool;

    /// crate the custom state based on the `FWSState` state and `emit`
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn custom_state(state: FWSState, emit: bool) -> (State<Self>, bool) {
    ///     (State::Custom(MyCustomType(state)), emit)
    /// }
    /// ```
    ///
    fn custom_state(state: FWSState, emit: bool) -> (State<Self>, bool);

    /// default impl. to handle the normal state of the `State` automaton
    ///
    /// It works following:
    ///
    /// 1. return `Ok((State::Normal, true))` if it is qtext in context of
    ///    `Self::ALLOW_UTF8` and `Self::OBS`
    /// 2. return `Ok(Self::custom_state(FWSState::HitCr, false))` if the input
    ///    was `'\r'`
    /// 3. else return `Err(CoreError::InvalidChar)`
    ///
    /// Note if `Self::ALLOW_UTF8` is set to true any `bch.as_u8() > 0x7f` will be treated
    /// as non-us-ascii utf8. This state machine does **not** validated if it is valid utf8
    /// so if it is used on a byte sequence which is not known to be a valid utf8 string it
    /// is still necessary to validate if it is utf8 and not e.g. latin1.
    fn handle_normal_state(bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
        let iu8 = bch.as_u8();

        let is_qtext_ws = if Self::OBS {
            MediaTypeChars::check_at(iu8 as usize, ObsQTextWs)
        } else {
            MediaTypeChars::check_at(iu8 as usize, QTextWs)
        };

        if is_qtext_ws || (Self::ALLOW_UTF8 && iu8 > 0x7f) {
            Ok((State::Normal, true))
        } else if iu8 == b'\r' {
            Ok(Self::custom_state(FWSState::HitCr, false))
        } else {
            Err(CoreError::InvalidChar)
        }
    }
}

/// A enum to represent the sate in a quoted string parser for specifications with FWS
///
/// FWS are forward white spaces, they can appear in media-types in the mime specification
/// a FWS is a `"\r\n"` seq followed by either `' '` or `'\t'`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FWSState {
    /// the state after the first `'\r'`
    HitCr,
    /// the state after `"\r\n"`
    HitNl,
    /// the state after `"\r\n "` or `"\r\n\t"` + any number of ws chars
    HadFws
}

impl FWSState {

    /// default implementation for handing FWSState state transitions
    ///
    /// Handles the state transition wrt. a given MimeParsingExt implementation.
    /// It assures that after a `\r` only `\n` can follow and after a `\n` either
    /// `' '` or `'\t'` has to follow. Lastly it makes sure that between two
    /// FWS there has to be at last one non ws character (at last in the non obs grammar).
    ///
    pub fn advance<Impl: MimeParsingExt>(self, bch: PartialCodePoint)
                                         -> Result<(State<Impl>, bool), CoreError>
    {
        use self::FWSState::*;
        let iu8 = bch.as_u8();
        match self {
            HitCr => {
                if iu8 == b'\n' {
                    Ok(Impl::custom_state(FWSState::HitNl, false))
                } else {
                    Err(CoreError::InvalidChar)
                }
            },
            HitNl => {
                if iu8 == b' ' || iu8 == b'\t' {
                    if Impl::OBS {
                        Ok((State::Normal, true))
                    } else {
                        //the new grammar does not allow ws-only lines, `obs-` one does
                        Ok(Impl::custom_state(FWSState::HadFws, true))
                    }
                } else {
                    Err(CoreError::InvalidChar)
                }
            },
            HadFws => {
                let lres = MediaTypeChars::lookup(iu8 as usize);
                // QText will be zero-sized so default etc. will be optimized awy
                let is_qtext = if Impl::OBS {
                    // we really should not ever end up in this branch as this state is
                    // meant to be used with non `obs-` grammar, but then it can be used
                    // differently too, and the if get's compiler optimized awy so
                    // it should be fine
                    QText.check(lres)
                } else {
                    ObsQText.check(lres)
                };
                if is_qtext || (Impl::ALLOW_UTF8 && iu8 > 0x7f) {
                    Ok((State::Normal, true))
                } else if Ws.check(lres) {
                    Ok(Impl::custom_state(FWSState::HadFws, true))
                } else if iu8 == b'"' {
                    Ok((State::End, false))
                } else if iu8 == b'\\' {
                    Ok((State::QPStart, false))
                } else {
                    Err(CoreError::InvalidChar)
                }
            }
        }
    }
}


