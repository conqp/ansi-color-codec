use crate::color_code::AnsiColorCode;
use crate::constants::{CODE_START, MAX_DIGITS, NUMBER_PREFIX, NUMBER_SUFFIX};
use crate::error::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Eq, PartialEq)]
pub struct BytesToAnsiColorCodesIterator<T>
where
    T: Iterator<Item = u8>,
{
    bytes: T,
}

impl<T> BytesToAnsiColorCodesIterator<T>
where
    T: Iterator<Item = u8>,
{
    fn next_header(&mut self) -> Option<Result<(), Error>> {
        match self.bytes.next() {
            Some(byte) => {
                if byte == CODE_START {
                    self.bytes.next().map_or_else(
                        || Some(Err(Error::ByteStreamTerminatedUnexpectedly)),
                        |byte| {
                            if byte as char == NUMBER_PREFIX {
                                Some(Ok(()))
                            } else {
                                Some(Err(Error::InvalidNumberPrefix(byte)))
                            }
                        },
                    )
                } else {
                    Some(Err(Error::InvalidStartByte(byte)))
                }
            }
            None => None,
        }
    }

    fn read_digits(&mut self) -> Result<String, Error> {
        let mut digits = String::new();

        for count in 0..=MAX_DIGITS {
            match self.bytes.next() {
                Some(byte) => {
                    if byte.is_ascii_digit() {
                        if count < MAX_DIGITS {
                            digits.push(byte as char);
                        } else {
                            return Err(Error::TooManyCodeDigits {
                                at_least: count,
                                max: MAX_DIGITS,
                            });
                        }
                    } else if byte as char == NUMBER_SUFFIX {
                        return if digits.is_empty() {
                            Err(Error::NoCodeDigitsFound)
                        } else {
                            Ok(digits)
                        };
                    } else {
                        return Err(Error::UnexpectedByte(byte));
                    }
                }
                None => return Err(Error::ByteStreamTerminatedUnexpectedly),
            }
        }

        Ok(digits)
    }

    fn parse_color_code(&mut self) -> Result<u8, Error> {
        let digits = self.read_digits()?;
        self.bytes.next(); // Discard bg-color encoded char
        digits
            .parse::<u8>()
            .map_or_else(|_| Err(Error::InvalidCodeValue(digits)), Ok)
    }
}

impl<T> From<T> for BytesToAnsiColorCodesIterator<T>
where
    T: Iterator<Item = u8>,
{
    fn from(bytes: T) -> Self {
        Self { bytes }
    }
}

impl<T> Iterator for BytesToAnsiColorCodesIterator<T>
where
    T: Iterator<Item = u8>,
{
    type Item = Result<AnsiColorCode, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(msg) = self.next_header()? {
            return Some(Err(msg));
        }

        match self.parse_color_code() {
            Ok(sum) => {
                if sum == 0 {
                    None
                } else {
                    Some(AnsiColorCode::new(sum))
                }
            }
            Err(error) => Some(Err(error)),
        }
    }
}
