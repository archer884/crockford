use error::*;

const BASE: u64 = 32;

pub fn decode<T: AsRef<str>>(input: T) -> Result<u64> {
    let input = input.as_ref();
    match input.len() {
        0 => return Err(Error::new(Kind::EmptyString, "Encoded input string is empty.")),
        n if n > 13 => return Err(Error::new(Kind::OutOfRange, "Encoded value is too large")),
        _ => match normalize_digits(input) {
            Err(e) => Err(e),
            Ok(digits) => {
                let mut n = 0;
                let mut base = 1;

                for &value in digits.iter().rev() {
                    n += (value as u64) * base;
                    base *= BASE;
                }

                Ok(n)
            }
        }
    }
}

fn normalize_digits(s: &str) -> Result<Vec<u8>> {
    s.bytes()
        .enumerate()
        .map(|(i, u)| to_normal_digit(i, u)).collect()
}

/// Attempts to convert an ascii digit to a normalized form.
fn to_normal_digit(idx: usize, u: u8) -> Result<u8> {
    const INT_OFFSET: u8 = b'0';
    const UPPERCASE_OFFSET: u8 = b'A' - 6;
    const LOWERCASE_OFFSET: u8 = b'a' - 6;

    match u {
        b'0' | b'O' | b'o' => Ok(0),
        b'1' | b'I' | b'i' | b'L' | b'l' => Ok(1),

        // It is not valid to convert O or L using the following, but those cases
        // should never actually fall through to this point.
        u @ b'0'...b'9' => Ok(u - INT_OFFSET),
        u @ b'A'...b'Z' => Ok(u - UPPERCASE_OFFSET),
        u @ b'a'...b'z' => Ok(u - LOWERCASE_OFFSET),

        _ => Err(Error::new(Kind::InvalidDigit(idx, u), "Invalid encoded digit."))
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
}