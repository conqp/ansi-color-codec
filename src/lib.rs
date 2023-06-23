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
const UNEXPECTED_TERMINATION_MSG: &str = "Byte stream terminated unexpectedly";

/// Print this str to reset a color code sequence on the terminal
pub const RESET: &str = "\x1b[0m ";

type ColorCodes<T> = FlatMap<T, [ColorCode; 2], fn(u8) -> [ColorCode; 2]>;

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
    /// let reference: Vec<ColorCode> = codes.iter().map(
    ///     |&code| ColorCode::new(code).unwrap()
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
    ///     .map(|color| color.unwrap())
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
    ///     .map(|result| result.unwrap() as char)
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
        self.flat_map(|byte| byte.to_color_codes())
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
    pub fn new(number: u8) -> Result<Self, String> {
        if (0..=COLOR_OFFSET_LOW + COLOR_CODE_LOW_MAX).contains(&number)
            || (COLOR_OFFSET_HIGH..=COLOR_OFFSET_HIGH + COLOR_CODE_LOW_MAX).contains(&number)
        {
            Ok(Self { number })
        } else {
            Err(format!("Invalid color code: {number}"))
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
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= COLOR_CODE_LOW_MAX {
            Self::new(value + COLOR_OFFSET_LOW)
        } else if value <= COLOR_CODE_MAX {
            Self::new((value & MASK_TRIPLET) + COLOR_OFFSET_HIGH)
        } else {
            Err(format!("Value out of bounds for color code: {value}"))
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

trait ColorEncodable {
    fn to_color_codes(&self) -> [ColorCode; 2];
    fn from_color_codes(color_codes: [ColorCode; 2]) -> Self;
}

impl ColorEncodable for u8 {
    fn to_color_codes(&self) -> [ColorCode; 2] {
        [
            ColorCode::try_from((self & MASK_HIGH) >> MASK_BITS)
                .map_or_else(|_| unreachable!(), |high| high),
            ColorCode::try_from(self & MASK_LOW).map_or_else(|_| unreachable!(), |low| low),
        ]
    }

    fn from_color_codes(color_codes: [ColorCode; 2]) -> Self {
        (color_codes[0].normalized() << MASK_BITS) + color_codes[1].normalized()
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
    fn next_header(&mut self) -> Option<Result<(), String>> {
        match self.bytes.next() {
            Some(byte) => {
                if byte == CODE_START {
                    self.bytes.next().map_or_else(
                        || Some(Err(UNEXPECTED_TERMINATION_MSG.to_string())),
                        |byte| {
                            if byte as char == NUMBER_PREFIX {
                                Some(Ok(()))
                            } else {
                                Some(Err(format!("Invalid number prefix: {byte}")))
                            }
                        },
                    )
                } else {
                    Some(Err(format!("Invalid start byte: {byte}")))
                }
            }
            None => None,
        }
    }

    fn read_digits(&mut self) -> Result<String, String> {
        let mut digits = String::new();

        for count in 0..=MAX_DIGITS {
            match self.bytes.next() {
                Some(byte) => {
                    if byte.is_ascii_digit() {
                        if count < MAX_DIGITS {
                            digits.push(byte as char);
                        } else {
                            return Err(format!("Expected at most {MAX_DIGITS} digits"));
                        }
                    } else if byte as char == NUMBER_SUFFIX {
                        return if digits.is_empty() {
                            Err("Expected at least one digit".to_string())
                        } else {
                            Ok(digits)
                        };
                    } else {
                        return Err(format!("Encountered Unexpected byte \"{byte}\""));
                    }
                }
                None => return Err(UNEXPECTED_TERMINATION_MSG.to_string()),
            }
        }

        Ok(digits)
    }

    fn parse_color_code(&mut self) -> Result<u8, String> {
        let digits = self.read_digits()?;
        self.bytes.next(); // Discard bg-color encoded char
        digits
            .parse::<u8>()
            .map_or_else(|_| Err(format!("Could not parse u8 from {digits}")), Ok)
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
    type Item = Result<ColorCode, String>;

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
            Err(msg) => Some(Err(format!("{msg} while parsing color code"))),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ColorCodesToBytes<T>
where
    T: Iterator<Item = Result<ColorCode, String>>,
{
    codes: T,
}

impl<T> From<T> for ColorCodesToBytes<T>
where
    T: Iterator<Item = Result<ColorCode, String>>,
{
    fn from(codes: T) -> Self {
        Self { codes }
    }
}

impl<T> Iterator for ColorCodesToBytes<T>
where
    T: Iterator<Item = Result<ColorCode, String>>,
{
    type Item = Result<u8, String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.codes.next() {
            Some(high) => match high {
                Ok(high) => self.codes.next().map_or_else(
                    || Some(Err("Missing second color code block".to_string())),
                    |low| match low {
                        Ok(low) => Some(Ok(u8::from_color_codes([high, low]))),
                        Err(msg) => Some(Err(msg)),
                    },
                ),
                Err(msg) => Some(Err(msg)),
            },
            None => None,
        }
    }
}
