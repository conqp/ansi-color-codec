use crate::code::Code;
use crate::constants::{CODE_START, NUMBER_PREFIX, NUMBER_SUFFIX};
use crate::error::Error;

const ASCII_DIGIT_MASK: u8 = 0b0011_0000;
const MAX_DIGITS: usize = 3;
const MULTIPLIERS: [u8; MAX_DIGITS] = [1, 10, 100];
type Digits = [Option<u8>; MAX_DIGITS];

#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Parser<T> {
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
        let digits = self.parse_color_code()?;
        self.bytes.next(); // Discard bg-color encoded char
        Ok(digits)
    }

    fn parse_color_code(&mut self) -> Result<u8, Error> {
        let mut digits = [None, None, None];

        for digit in &mut digits {
            let Some(byte) = self.bytes.next() else {
                return Err(Error::ByteStreamTerminatedUnexpectedly);
            };

            if byte.is_ascii_digit() {
                *digit = Some(byte);
            } else {
                return validate(digits, byte);
            }
        }

        let Some(byte) = self.bytes.next() else {
            return Err(Error::ByteStreamTerminatedUnexpectedly);
        };

        validate(digits, byte)
    }
}

impl<T> From<T> for Parser<T> {
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

fn validate(digits: Digits, byte: u8) -> Result<u8, Error> {
    if byte as char != NUMBER_SUFFIX {
        return Err(Error::UnexpectedByte(byte));
    }

    if digits.iter().all(Option::is_none) {
        return Err(Error::NoCodeDigitsFound);
    }

    parse_digits(digits)
}

fn parse_digits(digits: Digits) -> Result<u8, Error> {
    let mut n = 0;

    for (digit, multiplier) in digits
        .into_iter()
        .flatten()
        .rev()
        .zip(MULTIPLIERS)
        .map(|(digit, multiplier)| (digit ^ ASCII_DIGIT_MASK, multiplier))
    {
        if let Some(value) = digit
            .checked_mul(multiplier)
            .and_then(|digit| digit.checked_add(n))
        {
            n = value;
        } else {
            return Err(Error::InvalidCodeValue);
        }
    }

    Ok(n)
}
