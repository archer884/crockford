use error::*;

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
                place >>= 5;
            }

            Ok(n)
        }
    }
}

/// Attempts to convert an ascii digit to a normalized form.
fn to_normal_digit(idx: usize, u: u8) -> Result<u8> {
    static VALUE_MAPPING: [i8; 256] = include!("../resources/u8-mapping.txt");

    unsafe {
        match *VALUE_MAPPING.get_unchecked(u as usize) {
            -1 => Err(Error::new(
                Kind::InvalidDigit(idx, u),
                "Invalid encoded digit.",
            )),
            -2 => Err(Error::new(
                Kind::CheckDigitUnsupported(idx, u),
                "Check digits not currently supported.",
            )),
            result => Ok(result as u8),
        }
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
