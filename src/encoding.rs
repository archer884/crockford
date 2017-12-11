use iterator::*;
use UPPERCASE_ENCODING;

/// A type that can be encoded from a `u64` via Crockford Base32.
pub trait Encode {
    /// Encodes a `u64` value as Crockford Base32.
    fn encode(input: u64) -> Self where Self: Sized;

    /// Encodes a `u64` value via Crockford Base32 into `self`.
    fn encode_from(&mut self, input: u64);
}

impl Encode for Vec<u8> {
    fn encode(input: u64) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.encode_from(input);
        buf
    }

    fn encode_from(&mut self, input: u64) {
        let fits: Vec<_> = FiveBitIterator::new(input).collect();
        self.extend(fits.iter().rev().map(|&fit| {
            UPPERCASE_ENCODING[fit as usize]
        }));
    }
}

impl Encode for String {
    fn encode(input: u64) -> String {
        // All bytes in UPPERCASE_ENCODING are valid ASCII
        unsafe { String::from_utf8_unchecked(Vec::encode(input)) }
    }

    fn encode_from(&mut self, input: u64) {
        unsafe {
            // All encoded bytes are valid ASCII
            self.as_mut_vec().encode_from(input);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_returns_zero() {
        let input = 0;
        let expected = "0";
        let actual = String::encode(input);

        assert_eq!(expected, &*actual);
    }

    #[test]
    fn large_value_returns_correct_large_value() {
        let input = 65535;
        let expected = "1ZZZ";
        let actual = String::encode(input);

        assert_eq!(expected, &*actual);
    }
}
