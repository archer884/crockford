use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
    message: &'static str,
}

impl Error {
    pub fn new(kind: Kind, message: &'static str) -> Error {
        Error { kind, message }
    }
}

#[derive(Debug)]
pub enum Kind {
    EmptyString,
    OutOfRange,
    InvalidDigit(usize, u8),
    CheckDigitUnsupported(usize, u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::InvalidDigit(idx, digit) => {
                write!(f, "{} Found at {}: {}", self.message, idx, digit)
            }
            _ => write!(f, "{}", self.message),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message
    }
}

#[cfg(test)]
impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        fn kind_value(kind: &Kind) -> i32 {
            match *kind {
                Kind::EmptyString => 1,
                Kind::OutOfRange => 2,
                Kind::InvalidDigit(..) => 3,
                Kind::CheckDigitUnsupported(..) => 4,
            }
        }

        kind_value(&self.kind) == kind_value(&other.kind)
    }
}
