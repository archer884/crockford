use error::*;
use UPPERCASE_ENCODING;

const BASE: u64 = 0x20;

/// Attempts to decode a Crockford Base32-encoded string into a `u64` value.
pub fn decode<T: AsRef<str>>(input: T) -> Result<u64> {
    let input = input.as_ref();
    match input.len() {
        0 => Err(Error::new(
            Kind::EmptyString,
            "Encoded input string is empty.",
        )),
        n if n > 13 => Err(Error::new(Kind::OutOfRange, "Encoded value is too large")),
        _ => {
            let mut place = BASE.pow(input.len() as u32 - 1);
            let mut n = 0;

            for (idx, u) in input.bytes().enumerate() {
                let digit = to_normal_digit(idx, u)?;
                n += u64::from(digit).wrapping_mul(place);

                // This compiles to >>= 5
                place /= BASE;
            }

            Ok(n)
        }
    }
}

/// Attempts to convert an ascii digit to a normalized form.
fn to_normal_digit(idx: usize, u: u8) -> Result<u8> {
    const INT_OFFSET: u8 = b'0';

    match u {
        b'O' | b'o' => Ok(0),
        b'I' | b'i' | b'L' | b'l' => Ok(1),

        // U and u are relegated to use in the implementation of check digits because their
        // presence is otherwise prone to producing accidentally obscene strings.
        b'U' | b'u' => Err(Error::new(
            Kind::CheckDigitUnsupported(idx, u),
            "Check digits not currently supported.",
        )),

        u @ b'0'...b'9' => Ok(u - INT_OFFSET),
        u => match UPPERCASE_ENCODING.binary_search(&(u & !32)) {
            Ok(idx) => Ok(idx as u8),
            _ => Err(Error::new(
                Kind::InvalidDigit(idx, u),
                "Invalid encoded digit.",
            )),
        },
    }
}

#[cfg(test)]
mod tests {
    use decoding::decode;
    use error::*;

    #[test]
    fn zero_length_strings_fail() {
        let input = "";
        let expected = Err(Error::new(Kind::EmptyString, "Don't care"));
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn long_strings_fail() {
        let input = "12345678910121";
        let expected = Err(Error::new(Kind::OutOfRange, "Don't care"));
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_bytes_fail() {
        let input = "fZZ!2";
        let expected = Err(Error::new(Kind::InvalidDigit(3, 33), "Don't care"));
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn zero_becomes_zero() {
        let input = "0";
        let expected = Ok(0);
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn large_values_become_large_values() {
        assert_eq!(Ok(65535), decode("1zzz"));
        assert_eq!(Ok(65535), decode("1ZZZ"));
    }

    #[test]
    fn map_to_0() {
        assert_eq!(Ok(0), decode("O"));
        assert_eq!(Ok(0), decode("o"));
    }

    #[test]
    fn map_to_1() {
        assert_eq!(Ok(1), decode("I"));
        assert_eq!(Ok(1), decode("i"));
        assert_eq!(Ok(1), decode("L"));
        assert_eq!(Ok(1), decode("l"));
    }

    #[test]
    fn z_equals_31() {
        assert_eq!(Ok(31), decode("z"));
        assert_eq!(Ok(31), decode("Z"));
    }

    #[test]
    fn q_equals_23() {
        assert_eq!(Ok(23), decode("q"));
        assert_eq!(Ok(23), decode("Q"));
    }

    #[test]
    fn four_z_q_works() {
        assert_eq!(Ok(5111), decode("4zq"));
        assert_eq!(Ok(5111), decode("4ZQ"));
    }

    #[test]
    fn max_value_works() {
        assert_eq!(Ok(18446744073709551615), decode("fzzzzzzzzzzzz"));
    }

    #[test]
    fn u_produces_an_error_instead_of_a_crash() {
        assert!(decode("iVuv").is_err());
        assert!(decode("iVUv").is_err());
    }
}

#[cfg(test)]
mod benchmarks {
    use test::{self, Bencher};

    #[bench]
    fn fzq(b: &mut Bencher) {
        b.iter(|| test::black_box(super::decode("fzq").unwrap()));
    }

    #[bench]
    fn fzq_uppercase(b: &mut Bencher) {
        b.iter(|| test::black_box(super::decode("FZQ").unwrap()));
    }

    #[bench]
    fn fzzzzzzzzzzzz(b: &mut Bencher) {
        b.iter(|| test::black_box(super::decode("fzzzzzzzzzzzz").unwrap()));
    }

    #[bench]
    fn fzzzzzzzzzzzz_uppercase(b: &mut Bencher) {
        b.iter(|| test::black_box(super::decode("FZZZZZZZZZZZZ").unwrap()));
    }
}
