use error::*;
use UPPERCASE_ENCODING;

const BASE: u64 = 32;

/// Attempts to decode a Crockford Base32-encoded string into a `u64` value.
pub fn decode<T: AsRef<str>>(input: T) -> Result<u64> {
    let input = input.as_ref();
    match input.len() {
        0 => Err(Error::new(
            Kind::EmptyString,
            "Encoded input string is empty.",
        )),
        n if n > 13 => Err(Error::new(Kind::OutOfRange, "Encoded value is too large")),
        _ => match normalize_digits(input) {
            Err(e) => Err(e),
            Ok(digits) => {
                let mut n = 0;
                let mut base = Some(1);

                for &value in digits.iter().rev() {
                    match base {
                        Some(x) => {
                            n += u64::from(value).wrapping_mul(x);
                            base = x.checked_mul(BASE);
                        }

                        None => {
                            return Err(Error::new(
                                Kind::OutOfRange,
                                "The encoded value is too large.",
                            ))
                        }
                    }
                }

                Ok(n)
            }
        },
    }
}

fn normalize_digits(s: &str) -> Result<Vec<u8>> {
    s.bytes()
        .enumerate()
        .map(|(i, u)| to_normal_digit(i, u))
        .collect()
}

/// Attempts to convert an ascii digit to a normalized form.
fn to_normal_digit(idx: usize, u: u8) -> Result<u8> {
    const INT_OFFSET: u8 = b'0';

    match u {
        // Here, we opt for a slightly non-kosher behavior: we accept invalid letters such as
        // i, I, l, L, o, and O, but we convert them into zero or one.
        b'O' | b'o' => Ok(0),
        b'I' | b'i' | b'L' | b'l' => Ok(1),

        // U and u are relegated to use in the implementation of check digits because their
        // presence is otherwise prone to producing accidentally obscene strings.
        b'U' | b'u' => Err(Error::new(
            Kind::CheckDigitUnsupported(idx, u),
            "Check digits not currently supported.",
        )),

        u @ b'0'...b'9' => Ok(u - INT_OFFSET),
        u @ b'A'...b'Z' => match UPPERCASE_ENCODING.binary_search(&u) {
            Ok(idx) => Ok(idx as u8),
            _ => unreachable!("Seriously, if you got here, there is a problem."),
        },

        u @ b'a'...b'z' => match UPPERCASE_ENCODING.binary_search(&(u & !32)) {
            Ok(idx) => Ok(idx as u8),
            _ => unreachable!("C'mon, guys, I'm not kidding. This isn't possible."),
        },

        _ => Err(Error::new(
            Kind::InvalidDigit(idx, u),
            "Invalid encoded digit.",
        )),
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
        let input = "1ZZZ";
        let expected = Ok(65535);
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lowercase_large_values_become_large_values() {
        let input = "1zzz";
        let expected = Ok(65535);
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lowercase_o_becomes_zero() {
        let input = "o";
        let expected = Ok(0);
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn uppercase_o_becomes_zero() {
        let input = "O";
        let expected = Ok(0);
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lowercase_i_becomes_one() {
        let input = "i";
        let expected = Ok(1);
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn uppercase_i_becomes_one() {
        let input = "I";
        let expected = Ok(1);
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lowercase_l_becomes_one() {
        let input = "l";
        let expected = Ok(1);
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn uppercase_l_becomes_one() {
        let input = "L";
        let expected = Ok(1);
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn zee_equals_31() {
        assert_eq!(Ok(31), decode("z"));
        assert_eq!(Ok(31), decode("Z"));
    }

    #[test]
    fn queue_equals_23() {
        assert_eq!(Ok(23), decode("q"));
        assert_eq!(Ok(23), decode("Q"));
    }

    #[test]
    fn four_zee_queue_works() {
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
