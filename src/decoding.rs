use crate::{Error, Result};

const BASE: u64 = 0x20;

/// Attempts to decode a Crockford Base32-encoded string into a `u64` value.
pub fn decode(input: impl AsRef<str>) -> Result<u64> {
    let input = input.as_ref();
    match input.len() {
        0 => Err(Error::EmptyString),
        n if n > 13 => Err(Error::OutOfRange),
        _ => decode_str(input),
    }
}

#[inline]
fn decode_str(input: &str) -> Result<u64> {
    let mut place = BASE.pow(input.len() as u32 - 1);
    let mut n: u64 = 0;

    for (idx, u) in input.bytes().enumerate() {
        let digit = to_normal_digit(idx, u)?;
        n = u64::from(digit)
            .checked_mul(place)
            .and_then(|m| n.checked_add(m))
            .ok_or(Error::OutOfRange)?;
        place >>= 5;
    }

    Ok(n)
}

/// Attempts to convert an ascii digit to a normalized form.
#[inline]
fn to_normal_digit(idx: usize, u: u8) -> Result<u8> {
    static VALUE_MAPPING: [i8; 256] = include!("../resources/u8-mapping.txt");
    unsafe {
        match VALUE_MAPPING.get_unchecked(u as usize) {
            -1 => Err(Error::InvalidDigit {
                index: idx,
                value: u,
            }),
            -2 => Err(Error::CheckDigitUnsupported {
                index: idx,
                value: u,
            }),
            &result => Ok(result as u8),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, Error};

    #[test]
    fn zero_length_strings_fail() {
        let input = "";
        let expected = Err(Error::EmptyString);
        let actual = decode(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn long_strings_fail() {
        let input = "12345678910121";
        let expected = Err(Error::OutOfRange);
        let actual = decode(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_bytes_fail() {
        let input = "fZZ!2";
        let expected = Err(Error::InvalidDigit {
            index: 3,
            value: 33,
        });
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

    #[test]
    /// This is not a valid u64: ZJ75K085CMJ1A
    fn issue_16() {
        assert!(decode("ZJ75K085CMJ1A").is_err());
    }
}
