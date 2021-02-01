use std::{error, fmt};

/// Represents an error in decoding.
#[derive(Debug)]
pub struct Error {
    kind: Kind,
    message: &'static str,
}

impl Error {
    pub(crate) fn new(kind: Kind, message: &'static str) -> Error {
        Error { kind, message }
    }
}

#[derive(Debug)]
pub enum Kind {
    CheckDigitUnsupported(usize, u8),
    EmptyString,
    InvalidDigit(usize, u8),
    OutOfRange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::InvalidDigit(idx, digit) => {
                write!(f, "{} Byte value {} at index {}", self.message, digit, idx)
            }
            _ => write!(f, "{}", self.message),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        fn kind_value(kind: &Kind) -> i32 {
            match kind {
                Kind::EmptyString => 1,
                Kind::OutOfRange => 2,
                Kind::InvalidDigit(..) => 3,
                Kind::CheckDigitUnsupported(..) => 4,
            }
        }

        kind_value(&self.kind) == kind_value(&other.kind)
    }
}
