media-type-impl-utils  [![Crates.io](https://img.shields.io/crates/v/media-type-impl-utils.svg)](https://crates.io/crates/media-type-impl-utils) [![media-type-impl-utils](https://docs.rs/media-type-impl-utils/badge.svg)](https://docs.rs/media-type-impl-utils) [![License](https://img.shields.io/badge/License-MIT%2FApache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![Build Status](https://travis-ci.org/1aim/media-type-impl-utils.svg?branch=master)](https://travis-ci.org/1aim/media-type-impl-utils)
=================================


Utilities for implementing media type parsers. This is mainly used
by the `media-type` crate but some of the grammar parts also apply
to other parts (e.g. quoted-strings in some mail headers) so it can
be usefull to be able to reuse them.

**Note: Currently is crate is rather unstable. It will still keep to
  semver but changing to a newer (braking) version might induce large
  api changes.**

License
=======
Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Contribution
------------
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.


Change Log
==========

- **0.2.1**
    - Added percent_encoding module providing percent-encoding `EncodingSet` implementations for
      Http and Mime tokens.

- **0.3.0**
    - use `quoted-string` v0.6
    - renamed crate
    - implement Default for 0-sized type provider structs