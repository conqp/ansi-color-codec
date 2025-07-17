use crate::code::Code;
use crate::constants::{CODE_START, NUMBER_PREFIX, NUMBER_SUFFIX};
use crate::error::Error;

const MAX_DIGITS: u8 = 3;
type Digits = heapless::String<{ MAX_DIGITS as usize }>;

#[derive(Debug, Eq, PartialEq)]
pub struct Parser<T>
where
    T: Iterator<Item = u8>,
{
    bytes: T,
}

impl<T> Parser<T>
where
    T: Iterator<Item = u8>,
{
    fn next_header(&mut self) -> Option<Result<(), Error>> {
        self.bytes.next().map(|byte| {
            if byte == CODE_START {
                self.bytes
                    .next()
                    .map_or(Err(Error::ByteStreamTerminatedUnexpectedly), |byte| {
                        if byte as char == NUMBER_PREFIX {
                            Ok(())
                        } else {
                            Err(Error::InvalidNumberPrefix(byte))
                        }
                    })
            } else {
                Err(Error::InvalidStartByte(byte))
            }
        })
    }

    fn read_color_code(&mut self) -> Result<u8, Error> {
        let digits = self.read_digits()?;
        self.bytes.next(); // Discard bg-color encoded char
        digits.parse::<u8>().map_err(Error::InvalidCodeValue)
    }

    fn read_digits(&mut self) -> Result<Digits, Error> {
        let mut digits = Digits::new();

        for count in 0..=MAX_DIGITS {
            match self.bytes.next() {
                Some(byte) => {
                    if collect_digits(&mut digits, byte, count)? {
                        return Ok(digits);
                    }
                }
                None => return Err(Error::ByteStreamTerminatedUnexpectedly),
            }
        }

        Ok(digits)
    }
}

impl<T> From<T> for Parser<T>
where
    T: Iterator<Item = u8>,
{
    fn from(bytes: T) -> Self {
        Self { bytes }
    }
}

impl<T> Iterator for Parser<T>
where
    T: Iterator<Item = u8>,
{
    type Item = Result<Code, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(msg) = self.next_header()? {
            return Some(Err(msg));
        }

        self.read_color_code().map_or_else(
            |error| Some(Err(error)),
            |sum| {
                if sum == 0 {
                    None
                } else {
                    Some(Code::try_from(sum))
                }
            },
        )
    }
}

fn collect_digits(digits: &mut Digits, byte: u8, count: u8) -> Result<bool, Error> {
    if byte.is_ascii_digit() {
        if count < MAX_DIGITS {
            digits
                .push(byte as char)
                .expect("Digit should fit into buffer.");
            Ok(false) // Not done
        } else {
            Err(Error::TooManyCodeDigits {
                at_least: count,
                max: MAX_DIGITS,
            })
        }
    } else if byte as char == NUMBER_SUFFIX {
        if digits.is_empty() {
            Err(Error::NoCodeDigitsFound)
        } else {
            Ok(true) // Done
        }
    } else {
        Err(Error::UnexpectedByte(byte))
    }
}
