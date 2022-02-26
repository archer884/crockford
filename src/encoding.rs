use crate::{Write, UPPERCASE_ENCODING};

/// Encodes a `u64` value as a Crockford Base32-encoded string.
pub fn encode(n: u64) -> String {
    // The longest possible representation of u64 in Base32 is 13 digits.
    let mut fits = Vec::with_capacity(13);
    encode_into(n, &mut fits);

    // UPPERCASE_ENCODING contains only ASCII bytes.
    unsafe { String::from_utf8_unchecked(fits) }
}

/// Encodes a `u64` value as Crockford Base32 and writes it to the provided output.
///
/// Either `String` or `Vec<u8>` will be accepted.
pub fn encode_into(mut n: u64, w: &mut impl Write) {
    /// Number of digits required to represent a fully-populated u64 value.
    const BASE32_DIGITS: usize = 13;

    // Used for the initial shift.
    const QUAD_SHIFT: usize = 60;
    const QUAD_RESET: usize = 4;

    // Used for all subsequent shifts.
    const FIVE_SHIFT: usize = 59;
    const FIVE_RESET: usize = 5;

    // Don't waste time on pointless work.
    if n == 0 {
        w.write(b'0');
        return;
    }

    // Start by getting the most significant four bits OR by eating any leading
    // zero bits. After the first four, these zero bits MUST be dropped in sets
    // of five bits. We must retain the number of zero bits dropped.
    let digits_dropped = match (n >> QUAD_SHIFT) as usize {
        // Eat leading zero-bits. Following the first four bits, this MUST be
        // done in increments of five bits.
        0 => {
            n <<= QUAD_RESET;
            let dropped = n.leading_zeros() / 5 * 5;
            n <<= dropped;
            dropped / 5
        }

        // Write value of first four bits.
        i => {
            n <<= QUAD_RESET;
            w.write(UPPERCASE_ENCODING[i]);
            0
        }
    };

    let remaining_digits = BASE32_DIGITS - digits_dropped as usize - 1;

    for _ in 0..remaining_digits {
        w.write(UPPERCASE_ENCODING[(n >> FIVE_SHIFT) as usize]);
        n <<= FIVE_RESET;
    }
}

#[cfg(test)]
mod tests {
    use std::str;

    use crate::{decode, encode, encode_into};

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

        assert_eq!(expected, &*actual, "{} != {}", expected, actual);
    }

    #[test]
    fn x5111_is_4zq() {
        assert_eq!("4ZQ", &*encode(5111));
    }

    #[test]
    fn x18446744073709551615_is_fzzzzzzzzzzzz() {
        assert_eq!("FZZZZZZZZZZZZ", &*encode(18446744073709551615));
    }

    #[test]
    fn large_odd_number() {
        // No, this is not a joke.
        let x = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;
        let y = decode(encode(x)).unwrap();

        assert_eq!(x, y);
    }

    #[test]
    fn large_even_number() {
        // No, this is not a joke.
        let x = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let y = decode(encode(x)).unwrap();

        assert_eq!(x, y);
    }

    #[test]
    fn tiny_number() {
        let x = 1;
        let y = decode(encode(x)).unwrap();

        assert_eq!(x, y);
    }

    #[ignore = "This test takes forever to run."]
    #[test]
    fn round_trips() {
        let mut s = Vec::new();

        for n in 0..20_000_001 {
            encode_into(n, &mut s);
            assert_eq!(n, decode(str::from_utf8(&s).unwrap()).unwrap());
            s.clear();
        }

        for n in (u64::max_value() - 20_000_000)..u64::max_value() {
            encode_into(n, &mut s);
            assert_eq!(n, decode(str::from_utf8(&s).unwrap()).unwrap());
            s.clear();
        }
    }
}
