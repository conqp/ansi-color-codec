use std::array::IntoIter;
use std::fmt::{Display, Formatter};
use std::iter::FlatMap;

const MASK_LOW: u8 = 0b0000_1111;
const MASK_HIGH: u8 = 0b1111_0000;
const MASK_BITS: u8 = 4;
const MASK_TRIPLET: u8 = MASK_LOW >> 1;
const COLOR_OFFSET_LOW: u8 = 40;
const COLOR_OFFSET_HIGH: u8 = 100;
const COLOR_CODE_LOW_MAX: u8 = MASK_TRIPLET;
const COLOR_CODE_MAX: u8 = MASK_LOW;
const COLOR_CODE_HIGH_BIT: u8 = 0b1000;
const MAX_DIGITS: u8 = 3;
const CODE_START: u8 = 0x1b;
const NUMBER_PREFIX: char = '[';
const NUMBER_SUFFIX: char = 'm';
const SPACE: char = ' ';

/// Print this str to reset a color code sequence on the terminal
pub const RESET: &str = "\x1b[0m ";

type ColorCodes<T> = FlatMap<T, ColorCodePair, fn(u8) -> ColorCodePair>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    ByteStreamTerminatedUnexpectedly,
    InvalidCodeValue(String),
    InvalidColorCode(u8),
    InvalidNumberPrefix(u8),
    InvalidStartByte(u8),
    MissingSecondColorCodeBlock,
    NoCodeDigitsFound,
    TooManyCodeDigits(u8),
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
            Self::InvalidColorCode(code) => write!(f, "invalid color code: {code:?}"),
            Self::InvalidNumberPrefix(prefix) => write!(f, "invalid number prefix: {prefix}"),
            Self::InvalidStartByte(byte) => write!(f, "invalid start byte: {byte:?}"),
            Self::MissingSecondColorCodeBlock => write!(f, "missing second code block"),
            Self::NoCodeDigitsFound => write!(f, "no code digits found"),
            Self::TooManyCodeDigits(at_least) => {
                write!(f, "too many code digits found: {at_least}+ / {MAX_DIGITS}")
            }
            Self::UnexpectedByte(byte) => write!(f, "unexpected byte: {byte:?}"),
            Self::ValueOutOfBounds(value) => write!(f, "value out of bounds: {value}"),
        }
    }
}

/// Gives u8 iterators the ability to en- / decode bytes to / from ANSI background colors
pub trait ColorCodec<T>: Sized
where
    T: Iterator<Item = u8>,
{
    /// Returns an iterator that encodes all bytes as ANSI background colors
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_color_codec::ColorCodec;
    ///
    /// let text = String::from("Hello world.");
    /// let reference: Vec<u8> = vec![
    ///     27, 91, 52, 52, 109, 32, 27, 91, 49, 48, 48, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91,
    ///     52, 53, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 52, 109, 32, 27, 91, 52, 54,
    ///     109, 32, 27, 91, 49, 48, 52, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 55, 109,
    ///     32, 27, 91, 52, 50, 109, 32, 27, 91, 52, 48, 109, 32, 27, 91, 52, 55, 109, 32, 27, 91,
    ///     52, 55, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 55, 109, 32, 27, 91, 52, 55,
    ///     109, 32, 27, 91, 52, 50, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 52, 109, 32,
    ///     27, 91, 52, 54, 109, 32, 27, 91, 52, 52, 109, 32, 27, 91, 52, 50, 109, 32, 27, 91, 49,
    ///     48, 54, 109, 32,
    /// ];
    /// let code: Vec<u8> = text
    ///     .bytes()
    ///     .ansi_color_encode()
    ///     .map(|color| color.to_string())
    ///     .collect::<String>()
    ///     .bytes()
    ///     .collect();
    /// assert_eq!(code, reference);
    /// ```
    fn ansi_color_encode(self) -> ColorCodes<T>;

    /// Returns an iterator that interprets all bytes of a u8 iterator as an ANSI color code
    /// sequence
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_color_codec::{ColorCode, ColorCodec};
    ///
    /// let codes: Vec<u8> = vec![
    ///     44, 100, 46, 45, 46, 104, 46, 104, 46, 107, 42, 40, 47, 47, 46, 107, 47, 42, 46, 104,
    ///     46, 44, 42, 106,
    /// ];
    /// let reference: Vec<ColorCode> = codes.iter().filter_map(
    ///     |&code| ColorCode::new(code).ok()
    /// ).collect();
    /// let code: [u8; 151] = [
    ///     27, 91, 52, 52, 109, 32, 27, 91, 49, 48, 48, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91,
    ///     52, 53, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 52, 109, 32, 27, 91, 52, 54,
    ///     109, 32, 27, 91, 49, 48, 52, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 55, 109,
    ///     32, 27, 91, 52, 50, 109, 32, 27, 91, 52, 48, 109, 32, 27, 91, 52, 55, 109, 32, 27, 91,
    ///     52, 55, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 55, 109, 32, 27, 91, 52, 55,
    ///     109, 32, 27, 91, 52, 50, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 52, 109, 32,
    ///     27, 91, 52, 54, 109, 32, 27, 91, 52, 52, 109, 32, 27, 91, 52, 50, 109, 32, 27, 91, 49,
    ///     48, 54, 109, 32
    /// ];
    /// let colors: Vec<ColorCode> = code
    ///     .into_iter()
    ///     .interpret_as_ansi_colors()
    ///     .filter_map(Result::ok)
    ///     .collect();
    /// assert_eq!(colors, reference);
    /// ```
    fn interpret_as_ansi_colors(self) -> ColorCodesFromBytes<T>;

    /// Returns an iterator that decodes all bytes interpreted as a sequence of ANSI background
    /// colors to raw bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_color_codec::ColorCodec;
    ///
    /// let text = String::from("Hello world.");
    /// let code: String = text
    ///     .bytes()
    ///     .ansi_color_encode()
    ///     .map(|color| color.to_string())
    ///     .collect();
    /// let decoded: String = code
    ///     .bytes()
    ///     .ansi_color_decode()
    ///     .filter_map(|result| result.map(|byte| byte as char).ok())
    ///     .collect();
    /// assert_eq!(text, decoded);
    /// ```
    fn ansi_color_decode(self) -> ColorCodesToBytes<ColorCodesFromBytes<T>> {
        self.interpret_as_ansi_colors().into()
    }
}

impl<T> ColorCodec<T> for T
where
    T: Iterator<Item = u8>,
{
    fn ansi_color_encode(self) -> ColorCodes<T> {
        self.flat_map(ColorCodePair::from)
    }

    fn interpret_as_ansi_colors(self) -> ColorCodesFromBytes<T> {
        self.into()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ColorCode {
    number: u8,
}

impl ColorCode {
    /// Creates a new color code
    /// # Errors
    /// * Returns a `ansi_color_codec::Error` if an error occurs.
    pub fn new(number: u8) -> Result<Self, Error> {
        if (0..=COLOR_OFFSET_LOW + COLOR_CODE_LOW_MAX).contains(&number)
            || (COLOR_OFFSET_HIGH..=COLOR_OFFSET_HIGH + COLOR_CODE_LOW_MAX).contains(&number)
        {
            Ok(Self { number })
        } else {
            Err(Error::InvalidColorCode(number))
        }
    }

    #[must_use]
    pub const fn normalized(&self) -> u8 {
        if self.number < COLOR_OFFSET_HIGH {
            self.number - COLOR_OFFSET_LOW
        } else {
            self.number - COLOR_OFFSET_HIGH + COLOR_CODE_HIGH_BIT
        }
    }
}

impl TryFrom<u8> for ColorCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= COLOR_CODE_LOW_MAX {
            Self::new(value + COLOR_OFFSET_LOW)
        } else if value <= COLOR_CODE_MAX {
            Self::new((value & MASK_TRIPLET) + COLOR_OFFSET_HIGH)
        } else {
            Err(Error::ValueOutOfBounds(value))
        }
    }
}

impl Display for ColorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            CODE_START as char, NUMBER_PREFIX, self.number, NUMBER_SUFFIX, SPACE
        )
    }
}

pub struct ColorCodePair {
    high: ColorCode,
    low: ColorCode,
}

impl From<u8> for ColorCodePair {
    fn from(value: u8) -> Self {
        Self {
            high: ColorCode::try_from((value & MASK_HIGH) >> MASK_BITS)
                .unwrap_or_else(|_| unreachable!()),
            low: ColorCode::try_from(value & MASK_LOW).unwrap_or_else(|_| unreachable!()),
        }
    }
}

impl From<ColorCodePair> for u8 {
    fn from(value: ColorCodePair) -> Self {
        (value.high.normalized() << MASK_BITS) + value.low.normalized()
    }
}

impl IntoIterator for ColorCodePair {
    type Item = ColorCode;
    type IntoIter = IntoIter<ColorCode, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.high, self.low].into_iter()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ColorCodesFromBytes<T>
where
    T: Iterator<Item = u8>,
{
    bytes: T,
}

impl<T> ColorCodesFromBytes<T>
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
                            return Err(Error::TooManyCodeDigits(count));
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

impl<T> From<T> for ColorCodesFromBytes<T>
where
    T: Iterator<Item = u8>,
{
    fn from(bytes: T) -> Self {
        Self { bytes }
    }
}

impl<T> Iterator for ColorCodesFromBytes<T>
where
    T: Iterator<Item = u8>,
{
    type Item = Result<ColorCode, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(msg) = self.next_header()? {
            return Some(Err(msg));
        }

        match self.parse_color_code() {
            Ok(sum) => {
                if sum == 0 {
                    None
                } else {
                    Some(ColorCode::new(sum))
                }
            }
            Err(error) => Some(Err(error)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ColorCodesToBytes<T>
where
    T: Iterator<Item = Result<ColorCode, Error>>,
{
    codes: T,
}

impl<T> From<T> for ColorCodesToBytes<T>
where
    T: Iterator<Item = Result<ColorCode, Error>>,
{
    fn from(codes: T) -> Self {
        Self { codes }
    }
}

impl<T> Iterator for ColorCodesToBytes<T>
where
    T: Iterator<Item = Result<ColorCode, Error>>,
{
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.codes.next() {
            Some(high) => match high {
                Ok(high) => self.codes.next().map_or_else(
                    || Some(Err(Error::MissingSecondColorCodeBlock)),
                    |low| match low {
                        Ok(low) => Some(Ok(u8::from(ColorCodePair { high, low }))),
                        Err(error) => Some(Err(error)),
                    },
                ),
                Err(error) => Some(Err(error)),
            },
            None => None,
        }
    }
}
