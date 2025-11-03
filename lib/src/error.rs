use core::fmt::{self, Display, Formatter};

/// Encoding and decoding errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The input byte stream terminated prematurely.
    ByteStreamTerminatedUnexpectedly,
    /// An invalid color code value has been encountered.
    InvalidCodeValue,
    /// The prefix of the color code number was invalid.
    InvalidNumberPrefix(u8),
    /// An invalid start byte has been encountered.
    InvalidStartByte(u8),
    /// The second color code block is missing.
    MissingSecondColorCodeBlock,
    /// No digits for the color code number were found.
    NoCodeDigitsFound,
    /// Too many digits for the color code have been encountered.
    TooManyCodeDigits {
        /// Number of digits that have been processed.
        at_least: u8,
        /// Number of digits that were expected.
        max: u8,
    },
    /// An unexpected byte has been encountered.
    UnexpectedByte(u8),
    /// The given color code value was out of bounds.
    ValueOutOfBounds(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ByteStreamTerminatedUnexpectedly => {
                write!(f, "byte stream terminated unexpectedly")
            }
            Self::InvalidCodeValue => write!(f, "invalid code value"),
            Self::InvalidNumberPrefix(prefix) => write!(f, "invalid number prefix: {prefix}"),
            Self::InvalidStartByte(byte) => write!(f, "invalid start byte: {byte:X?}"),
            Self::MissingSecondColorCodeBlock => write!(f, "missing second code block"),
            Self::NoCodeDigitsFound => write!(f, "no code digits found"),
            Self::TooManyCodeDigits { at_least, max } => {
                write!(f, "too many code digits found: {at_least}+ / {max}")
            }
            Self::UnexpectedByte(byte) => write!(f, "unexpected byte: {byte:X?}"),
            Self::ValueOutOfBounds(value) => write!(f, "value out of bounds: {value}"),
        }
    }
}

impl core::error::Error for Error {}
