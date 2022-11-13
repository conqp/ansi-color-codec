use std::iter::{Map, SkipWhile};

const MASK_LOW: u8 = 0b00001111;
const MASK_HIGH: u8 = 0b11110000;
const COLOR_OFFSET_LOW: u8 = 40;
const COLOR_OFFSET_HIGH: u8 = 100;
const OFFSET_THRESHOLD: u8 = 8;
const CODE_START: u8 = 0x1b;
const NUMBER_PREFIX: char = '[';
const NUMBER_SUFFIX: char = 'm';
const UNEXPECTED_TERM: &str = "Byte stream terminated unexpectedly";

pub const CLEARING_CODE: &str = "\x1b[0m";

type ByteFilter = fn(&u8) -> bool;
type ColorUnwrapper = fn(Result<ColorCode, String>) -> ColorCode;
type DecodedColors<T> =
    ColorCodesToBytes<Map<ColorCodesFromBytes<SkipWhile<T, ByteFilter>>, ColorUnwrapper>>;

pub trait ColorCodec<T>
where
    T: Iterator<Item = u8>,
{
    fn color_code(self) -> BytesToColorCodes<T>;
    fn color_decode(self) -> DecodedColors<T>;
}

impl<T> ColorCodec<T> for T
where
    T: Iterator<Item = u8>,
{
    fn color_code(self) -> BytesToColorCodes<T> {
        BytesToColorCodes::from(self)
    }

    fn color_decode(self) -> DecodedColors<T> {
        ColorCodesToBytes::from(
            ColorCodesFromBytes::from(self.skip_while((|item| *item != CODE_START) as ByteFilter))
                .map((|result| result.unwrap()) as ColorUnwrapper),
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ColorCode {
    number: u8,
}

impl ColorCode {
    pub fn new(number: u8) -> Self {
        Self { number }
    }

    pub fn byte(&self) -> u8 {
        if self.number < COLOR_OFFSET_HIGH {
            self.number - COLOR_OFFSET_LOW
        } else {
            self.number - COLOR_OFFSET_HIGH
        }
    }
}

impl From<u8> for ColorCode {
    fn from(byte: u8) -> Self {
        if byte < OFFSET_THRESHOLD {
            Self::new(byte + COLOR_OFFSET_LOW)
        } else {
            Self::new(byte + COLOR_OFFSET_HIGH)
        }
    }
}

impl ToString for ColorCode {
    fn to_string(&self) -> String {
        format!("\x1b[{}m ", self.number)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ColorCodesFromBytes<T>
where
    T: Iterator<Item = u8>,
{
    bytes: T,
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
        let mut digits = Vec::new();

        match self.bytes.next() {
            Some(byte) => {
                if byte != CODE_START {
                    return Some(Err(format!("Invalid start byte: {}", byte)));
                }
            }
            None => return None,
        }

        match self.bytes.next() {
            Some(byte) => {
                if byte as char != NUMBER_PREFIX {
                    return Some(Err(format!("Invalid number prefix: {}", byte)));
                }
            }
            None => return Some(Err(UNEXPECTED_TERM.to_string())),
        }

        loop {
            match self.bytes.next() {
                Some(byte) => {
                    if byte.is_ascii_digit() {
                        digits.push(byte);
                    } else if byte as char == NUMBER_SUFFIX {
                        break;
                    } else {
                        return Some(Err(format!(
                            "Unexpected byte while parsing digits: {}",
                            byte
                        )));
                    }
                }
                None => return Some(Err(UNEXPECTED_TERM.to_string())),
            }
        }

        self.bytes.next();

        match digits
            .iter()
            .rev()
            .enumerate()
            .map(|(index, digit)| (digit & MASK_LOW) * 10_u8.pow(index as u32))
            .sum()
        {
            0 => None,
            sum => Some(Ok(ColorCode::new(sum))),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ColorCodesToBytes<T>
where
    T: Iterator<Item = ColorCode>,
{
    codes: T,
}

impl<T> From<T> for ColorCodesToBytes<T>
where
    T: Iterator<Item = ColorCode>,
{
    fn from(codes: T) -> Self {
        Self { codes }
    }
}

impl<T> Iterator for ColorCodesToBytes<T>
where
    T: Iterator<Item = ColorCode>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.codes.next() {
            Some(high) => match self.codes.next() {
                Some(low) => Some((high.byte() << 4) + low.byte()),
                None => Some(high.byte() << 4),
            },
            None => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct BytesToColorCodes<T>
where
    T: Iterator<Item = u8>,
{
    bytes: T,
    current: Option<u8>,
}

impl<T> From<T> for BytesToColorCodes<T>
where
    T: Iterator<Item = u8>,
{
    fn from(bytes: T) -> Self {
        Self {
            bytes,
            current: None,
        }
    }
}

impl<T> Iterator for BytesToColorCodes<T>
where
    T: Iterator<Item = u8>,
{
    type Item = ColorCode;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(byte) => {
                self.current = None;
                Some(ColorCode::from(byte & MASK_LOW))
            }
            None => match self.bytes.next() {
                Some(byte) => {
                    self.current = Some(byte);
                    Some(ColorCode::from((byte & MASK_HIGH) >> 4))
                }
                None => None,
            },
        }
    }
}
