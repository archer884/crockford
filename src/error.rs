use std::{error, fmt};

/// Represents an error in decoding.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    CheckDigitUnsupported { index: usize, value: u8 },
    EmptyString,
    InvalidDigit { index: usize, value: u8 },
    OutOfRange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CheckDigitUnsupported { index, value } => write!(
                f,
                "check digits not supported (byte b{:?} at {index})",
                *value as char
            ),
            Error::EmptyString => f.write_str("attempt to decode empty string"),
            Error::InvalidDigit { index, value } => {
                write!(f, "invalid digt (byte b{:?} at {index}", *value as char)
            }
            Error::OutOfRange => f.write_str("encoded value too large"),
        }
    }
}

impl error::Error for Error {}
