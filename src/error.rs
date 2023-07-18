use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    ByteStreamTerminatedUnexpectedly,
    InvalidCodeValue(ParseIntError),
    InvalidNumberPrefix(u8),
    InvalidStartByte(u8),
    MissingSecondColorCodeBlock,
    NoCodeDigitsFound,
    TooManyCodeDigits { at_least: u8, max: u8 },
    UnexpectedByte(u8),
    ValueOutOfBounds(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ByteStreamTerminatedUnexpectedly => {
                write!(f, "byte stream terminated unexpectedly")
            }
            Self::InvalidCodeValue(value) => write!(f, "invalid code value: {value}"),
            Self::InvalidNumberPrefix(prefix) => write!(f, "invalid number prefix: {prefix}"),
            Self::InvalidStartByte(byte) => write!(f, "invalid start byte: {byte:?}"),
            Self::MissingSecondColorCodeBlock => write!(f, "missing second code block"),
            Self::NoCodeDigitsFound => write!(f, "no code digits found"),
            Self::TooManyCodeDigits { at_least, max } => {
                write!(f, "too many code digits found: {at_least}+ / {max}")
            }
            Self::UnexpectedByte(byte) => write!(f, "unexpected byte: {byte:?}"),
            Self::ValueOutOfBounds(value) => write!(f, "value out of bounds: {value}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::InvalidCodeValue(error) = self {
            Some(error)
        } else {
            None
        }
    }
}
