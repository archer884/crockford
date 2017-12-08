mod error;
mod iterator;

mod encoding;
mod decoding;

pub use encoding::encode;
pub use decoding::decode;

static UPPERCASE_ENCODING: &'static [u8] = &[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'V', b'W', b'X', b'Y', b'Z'];
