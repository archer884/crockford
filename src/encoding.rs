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
pub fn encode_into(n: u64, w: &mut impl Write) {
    /// Mask for the first four bits.
    const MASK_4: u64 = 15;

    /// Mask for sets of five bits.
    const MASK_5: u64 = 31;

    if n == 0 {
        w.write(b'0');
        return;
    }

    // We're gonna use this array as scratch space.
    let mut buf = [0u8; 13];
    
    buf[0] = read_digit(n, 60, MASK_4);
    buf[1] = read_digit(n, 55, MASK_5);
    buf[2] = read_digit(n, 50, MASK_5);
    buf[3] = read_digit(n, 45, MASK_5);
    buf[4] = read_digit(n, 40, MASK_5);
    buf[5] = read_digit(n, 35, MASK_5);
    buf[6] = read_digit(n, 30, MASK_5);
    buf[7] = read_digit(n, 25, MASK_5);
    buf[8] = read_digit(n, 20, MASK_5);
    buf[9] = read_digit(n, 15, MASK_5);
    buf[10] = read_digit(n, 10, MASK_5);
    buf[11] = read_digit(n, 5, MASK_5);
    buf[12] = read_digit(n, 0, MASK_5);

    for &u in buf.iter().skip_while(|&&u| u == b'0') {
        w.write(u);
    }
}

#[inline]
fn read_digit(n: u64, shift: usize, mask: u64) -> u8 {
    UPPERCASE_ENCODING[((n >> shift) & mask) as usize]
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
