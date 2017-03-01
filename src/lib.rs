mod error;
mod iterator;

mod encoding;
mod decoding;

pub use encoding::encode;
pub use decoding::decode;

static UPPERCASE_ENCODING: &'static [u8] = &[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'V', b'W', b'X', b'Y', b'Z'];
// static LOWERCASE_ENCODING: &'static [u8] = &[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'j', b'k', b'm', b'n', b'p', b'q', b'r', b's', b't', b'u', b'v', b'x', b'y', b'z'];
