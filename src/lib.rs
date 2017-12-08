//! # Crockford
//!
//! This library is intended to provide an easy way to encode and decode identifiers 
//! (large integers) as Crockford-encoded strings. If you want to encode or decode arbitrary
//! data, this probably is not the library for you. (But there totally is another one.)
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
//! ## Decoding
//!
//! Use the decode function to decode Crockford Base32-encoded strings. This operation can fail;
//! if it does, you'll get a reasonably useful error instead of a number.
//!
//! ```rust
//! let x = crockford::decode("4zq");
//! let y = crockford::decode("4ZQ");
//!
//! assert_eq!(5111, x.unwrap());
//! assert_eq!(5111, y.unwrap());
//! ```

mod error;
mod iterator;

mod encoding;
mod decoding;

pub use encoding::encode;
pub use decoding::decode;
pub use error::Error;

static UPPERCASE_ENCODING: &[u8] = &[
    b'0',
    b'1',
    b'2',
    b'3',
    b'4',
    b'5',
    b'6',
    b'7',
    b'8',
    b'9',
    b'A',
    b'B',
    b'C',
    b'D',
    b'E',
    b'F',
    b'G',
    b'H',
    b'J',
    b'K',
    b'M',
    b'N',
    b'P',
    b'Q',
    b'R',
    b'S',
    b'T',
    b'V',
    b'W',
    b'X',
    b'Y',
    b'Z',
];
