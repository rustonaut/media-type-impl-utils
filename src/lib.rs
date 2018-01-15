/// provides a number of utilities for writing media-type parsers in rust
///
/// Utils:
///
/// - `quoted_string`: provides implementations for a number of traits from the `quoted-string`
///   crate including `ParsingImpl`, `QuotingClassifier` and `WithoutQuotingValidator`. Implementations
///   are provided for a number of different use cases including media-types in http, mime as well as
///   a more strict impl for media types compatible with all other implementations and a impl being
///   usable with media-types compatible with any (/at last one) of the other implementations.
///
/// - `lookup_table`: provides a `lut` lookup table for bytes/us-ascii chars used in context of
///   media-type parsing.
#[macro_use]
extern crate lut;
extern crate quoted_string as qs;

pub mod lookup_tables;
pub mod quoted_string;