//! # Crockford
//!
//! This library is intended to provide an easy way to encode and decode identifiers
//! (large integers) as [Crockford-encoded](https://www.crockford.com/wrmg/base32.html)
//! strings. If you want to encode or decode arbitrary data,
//! [another library](https://docs.rs/base32) is probably a better choice.
//!
//! ## Encoding
//!
//! Use the encode function to encode `u64` values into Crockford Base32-encoded strings. This
//! operation cannot fail, so you will always get back a string rather than any kind of result
//! value.
//!
//! ```rust
//! let x = crockford::encode(5111);
//!
//! assert_eq!("4ZQ", &*x);
//! ```
//!
//! ### Faster encoding (aka "Plan B")
//!
//! Because this is Rust, particular focus is given to runtime efficiency--or, at least, allowing
//! the user to achieve runtime efficiency. As a result, we provide a second, more complicated
//! encoding option.
//!
//! ```rust
//! # use crockford;
//! # use std::str;
//! # fn run() -> Result<(), str::Utf8Error> {
//! // The longest possible representation of u64 is 13 digits.
//! let mut buf = Vec::with_capacity(13);
//! crockford::encode_into(5111, &mut buf);
//!
//! let result = std::str::from_utf8(&buf)?;
//! assert_eq!("4ZQ", result);
//! # Ok(())
//! # }
//! ```
//!
//! This `encode_into` method also accepts `&mut String`, if you prefer.
//!
//! ## Decoding
//!
//! Use the decode function to decode Crockford Base32-encoded strings. This operation can fail;
//! if it does, you'll get a reasonably useful error instead of a number.
//!
//! ```rust
//! # use crockford::{self, Error};
//! # fn run() -> Result<(), Error> {
//! let x = crockford::decode("4zq");
//! let y = crockford::decode("4ZQ");
//!
//! assert_eq!(5111, x?);
//! assert_eq!(5111, y?);
//! # Ok(())
//! # }
//! # run().unwrap()
//! ```

mod decoding;
mod encoding;
mod error;

pub use decoding::decode;
pub use encoding::*;
pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

static UPPERCASE_ENCODING: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

