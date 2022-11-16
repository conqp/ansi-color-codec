use std::iter::FlatMap;

const MASK_LOW: u8 = 0b00001111;
const MASK_HIGH: u8 = 0b11110000;
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
const UNEXPECTED_TERMINATION_MSG: &str = "Byte stream terminated unexpectedly";

type ColorCodes<T> = FlatMap<T, [ColorCode; 2], fn(u8) -> [ColorCode; 2]>;

pub trait ColorCodec<T>
where
    T: Iterator<Item = u8>,
{
    fn ansi_color_encode(self) -> ColorCodes<T>;
    fn ansi_color_decode(self) -> ColorCodesToBytes<ColorCodesFromBytes<T>>;
}

impl<T> ColorCodec<T> for T
where
    T: Iterator<Item = u8>,
{
    fn ansi_color_encode(self) -> ColorCodes<T> {
        self.flat_map(|byte| byte.to_color_codes())
    }

    fn ansi_color_decode(self) -> ColorCodesToBytes<ColorCodesFromBytes<T>> {
        ColorCodesToBytes::from(ColorCodesFromBytes::from(self))
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
            Err(format!("Invalid color code: {}", number))
        }
    }

    pub fn normalized(&self) -> u8 {
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
            Err(format!("Value out of bounds for color code: {}", value))
        }
    }
}

impl ToString for ColorCode {
    fn to_string(&self) -> String {
        format!("\x1b[{}m ", self.number)
    }
}

trait ColorEncodable {
    fn to_color_codes(&self) -> [ColorCode; 2];
    fn from_color_codes(color_codes: [ColorCode; 2]) -> Self;
}

impl ColorEncodable for u8 {
    fn to_color_codes(&self) -> [ColorCode; 2] {
        [
            ColorCode::try_from((self & MASK_HIGH) >> MASK_BITS).unwrap(),
            ColorCode::try_from(self & MASK_LOW).unwrap(),
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
                    match self.bytes.next() {
                        Some(byte) => {
                            if byte as char == NUMBER_PREFIX {
                                Some(Ok(()))
                            } else {
                                Some(Err(format!("Invalid number prefix: {}", byte)))
                            }
                        }
                        None => Some(Err(UNEXPECTED_TERMINATION_MSG.to_string())),
                    }
                } else {
                    Some(Err(format!("Invalid start byte: {}", byte)))
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
                            return Err(format!("Expected at most {} digits", MAX_DIGITS));
                        }
                    } else if byte as char == NUMBER_SUFFIX {
                        return if digits.is_empty() {
                            Err("Expected at least one digit".to_string())
                        } else {
                            Ok(digits)
                        };
                    } else {
                        return Err(format!("Encountered Unexpected byte \"{}\"", byte));
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
        match digits.parse::<u8>() {
            Ok(number) => Ok(number),
            Err(_) => Err(format!("Could not parse u8 from {}", digits)),
        }
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
            Err(msg) => Some(Err(format!("{} while parsing color code", msg))),
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
                Ok(high) => match self.codes.next() {
                    Some(low) => match low {
                        Ok(low) => Some(Ok(u8::from_color_codes([high, low]))),
                        Err(msg) => Some(Err(msg)),
                    },
                    None => Some(Err("Missing second color code block".to_string())),
                },
                Err(msg) => Some(Err(msg)),
            },
            None => None,
        }
    }
}
