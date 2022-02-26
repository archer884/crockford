/// Represents writable buffer capable of receiving encoded data.
///
/// Write is implemented on `Vec<u8>` and `String`, but you are free to implement it on your own
/// types. One conceivable purpose would be to allow for lowercase encoding output by inverting
/// the cap bit before writing.
pub trait Write {
    /// Writes a single byte (or, more precisely, a 5-bit group) to the output.
    fn write(&mut self, u: u8);
}

impl Write for String {
    fn write(&mut self, u: u8) {
        // UPPERCASE_ENCODING contains only ASCII bytes.
        unsafe {
            self.as_mut_vec().push(u);
        }
    }
}

impl Write for Vec<u8> {
    fn write(&mut self, u: u8) {
        self.push(u);
    }
}

/// Encodes a `u64` value as a Crockford Base32-encoded string.
pub fn encode(n: u64) -> String {
    // The longest possible representation of u64 in Base32 is 13 digits.
    let mut fits = Vec::with_capacity(13);
    //encode_into(n, &mut fits);
    encode_with_bitmasks(n, &mut fits);

    // UPPERCASE_ENCODING contains only ASCII bytes.
    unsafe { String::from_utf8_unchecked(fits) }
}

/// Encodes a `u64` value as Crockford Base32 and writes it to the provided output.
///
/// Either `String` or `Vec<u8>` will be accepted.
pub fn encode_into<T: Write>(mut n: u64, w: &mut T) {
    use crate::UPPERCASE_ENCODING;

    // Used for the initial shift.
    const QUAD_SHIFT: usize = 60;
    const QUAD_RESET: usize = 4;

    // Used for all subsequent shifts.
    const FIVE_SHIFT: usize = 59;
    const FIVE_RESET: usize = 5;

    // After we clear the four most significant bits, the four least significant bits will be
    // replaced with 0001. We can then know to stop once the four most significant bits are,
    // likewise, 0001.
    const STOP_BIT: u64 = 1 << QUAD_SHIFT;

    if n == 0 {
        w.write(b'0');
        return;
    }

    // Start by getting the most significant four bits. We get four here because these would be
    // leftovers when starting from the least significant bits. In either case, tag the four least
    // significant bits with our stop bit.
    match (n >> QUAD_SHIFT) as usize {
        // Eat leading zero-bits. This should not be done if the first four bits were non-zero.
        // Additionally, we *must* do this in increments of five bits.
        0 => {
            n <<= QUAD_RESET;
            n |= 1;
            n <<= n.leading_zeros() / 5 * 5;
        }

        // Write value of first four bytes.
        i => {
            n <<= QUAD_RESET;
            n |= 1;
            w.write(UPPERCASE_ENCODING[i]);
        }
    }

    // From now until we reach the stop bit, take the five most significant bits and then shift
    // left by five bits.
    while n != STOP_BIT {
        w.write(UPPERCASE_ENCODING[(n >> FIVE_SHIFT) as usize]);
        n <<= FIVE_RESET;
    }
}

fn encode_with_bitmasks<T: Write>(n: u64, w: &mut T) {
    use crate::REVERSED_ENCODING;
    use crate::REVERSE4_ENCODING;
    const MASK5: u64 = 31;
    const MASK4: u64 = 15;
    
    if n == 0 {
        w.write(b'0');
        return;
    }

    let mut shift = (((n.leading_zeros()+1)/5)+1)*5-1; // starting bitshift
    let r= n.reverse_bits();
    if shift == 4 {
        let value = r & MASK4;
        w.write(REVERSE4_ENCODING[value as usize]);
    } else if shift > 59 {
        shift = 59;
    } else {
        shift -= 5;
    }

    while shift < 63 {
        let value = (r>>shift) & MASK5;
        shift += 5;
        w.write(REVERSED_ENCODING[value as usize]);
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

        assert_eq!(expected, &*actual);
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

    // Test is ignored because it takes forever to run.
    #[ignore]
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

    #[test]
    fn test_leading_zeros(){
        let mut n=1u64;
        n = n.reverse_bits();
        for x in 0..64 {
            let actual = (((n.leading_zeros()+1)/5)+1)*5-1;  // make sure this is the same equation as above
            let expected: u32 = match x {
                0..=3 => 4,
                4..=8 => 9,
                9..=13 => 14,
                14..=18 => 19,
                19..=23 => 24,
                24..=28 => 29,
                29..=33 => 34,
                34..=38 => 39,
                39..=43 => 44,
                44..=48 => 49,
                49..=53 => 54,
                54..=58 => 59,
                59..=63 => 64,
                _ => 0
            };
            assert_eq!(expected, actual);
            n >>= 1;
        }
    }
}
