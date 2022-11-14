use std::iter::Map;

const MASK_LOW: u8 = 0b00001111;
const MASK_HIGH: u8 = 0b11110000;
const MASK_SIZE: u8 = 4;
const MASK_TRIPLET: u8 = MASK_LOW >> 1;
const COLOR_OFFSET_LOW: u8 = 40;
const COLOR_OFFSET_HIGH: u8 = 100;
const COLOR_CODE_LOW_MAX: u8 = MASK_TRIPLET;
const COLOR_CODE_MAX: u8 = MASK_LOW;
const COLOR_CODE_HIGH_BIT: u8 = 0b1000;
const CODE_START: u8 = 0x1b;
const NUMBER_BASE: u8 = 10;
const NUMBER_PREFIX: char = '[';
const NUMBER_SUFFIX: char = 'm';
const UNEXPECTED_TERMINATION_MSG: &str = "Byte stream terminated unexpectedly";
const UTF_8_BOM: [u8; 3] = [239, 187, 191];

type ColorUnwrapper = fn(Result<ColorCode, String>) -> ColorCode;
type DecodedColors<T> = ColorCodesToBytes<Map<ColorCodesFromBytes<T>, ColorUnwrapper>>;

pub trait ColorCodec<T>
where
    T: Iterator<Item = u8>,
{
    fn ansi_color_encode(self) -> BytesToColorCodes<T>;
    fn ansi_color_decode(self) -> DecodedColors<T>;
}

impl<T> ColorCodec<T> for T
where
    T: Iterator<Item = u8>,
{
    fn ansi_color_encode(self) -> BytesToColorCodes<T> {
        BytesToColorCodes::from(self)
    }

    fn ansi_color_decode(self) -> DecodedColors<T> {
        ColorCodesToBytes::from(
            ColorCodesFromBytes::from(self).map((|result| result.unwrap()) as ColorUnwrapper),
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
            Ok(Self::new(value + COLOR_OFFSET_LOW))
        } else if value <= COLOR_CODE_MAX {
            Ok(Self::new((value & MASK_TRIPLET) + COLOR_OFFSET_HIGH))
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
    fn skip_utf_8_bom(&mut self) -> Option<u8> {
        for bom in UTF_8_BOM {
            match self.bytes.next() {
                Some(byte) => {
                    if byte != bom {
                        return Some(byte);
                    }
                }
                None => return None,
            }
        }

        self.bytes.next()
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
        let mut digits = Vec::new();

        match self.skip_utf_8_bom() {
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
            None => return Some(Err(UNEXPECTED_TERMINATION_MSG.to_string())),
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
                None => return Some(Err(UNEXPECTED_TERMINATION_MSG.to_string())),
            }
        }

        self.bytes.next();

        match digits
            .iter()
            .rev()
            .enumerate()
            .map(|(index, digit)| (digit & MASK_LOW) * NUMBER_BASE.pow(index as u32))
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
                Some(low) => Some((high.normalized() << MASK_SIZE) + low.normalized()),
                None => Some(high.normalized() << MASK_SIZE),
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
                Some(ColorCode::try_from(byte & MASK_LOW).unwrap())
            }
            None => match self.bytes.next() {
                Some(byte) => {
                    self.current = Some(byte);
                    Some(ColorCode::try_from((byte & MASK_HIGH) >> MASK_SIZE).unwrap())
                }
                None => None,
            },
        }
    }
}
