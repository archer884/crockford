//! # Encoding
//!
//! Use the encode function to encode `u64` values into Crockford base 32-encoded strings. This
//! operation cannot fail, so you will always get back a string rather than any kind of result
//! value.
//!
//! ```rust
//! let x = crockford::encode(5111);
//!
//! assert_eq!("4ZQ", &*x);
//! ```

use iterator::*;
use UPPERCASE_ENCODING;

/// Encodes a `u64` value as a Crockford base 32-encoded string.
pub fn encode(input: u64) -> String {
    let mut fits: Vec<_> = FiveBitIterator::new(input).collect();
    let mut buf = String::new();
    while let Some(fit) = fits.pop() {
        buf.push(UPPERCASE_ENCODING[fit as usize] as char);
    }
    buf
}

#[cfg(test)]
mod tests {
    use encoding::encode;

    #[test]
    fn zero_returns_zero() {
        let input = 0;
        let expected = "0";
        let actual = encode(input);

        assert_eq!(expected, &*actual);
    }

    #[test]
    fn large_value_returns_correct_large_value() {
        let input = 65535;
        let expected = "1ZZZ";
        let actual = encode(input);

        assert_eq!(expected, &*actual);
    }
}
