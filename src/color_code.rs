use crate::constants::{CODE_START, MASK_BITS, NUMBER_PREFIX, NUMBER_SUFFIX};
use crate::error::Error;
use std::fmt::{Display, Formatter};

const CHAR_START: char = CODE_START as char;
const COLOR_CODE_HIGH_BIT: u8 = 0b1000;
const COLOR_CODE_HIGH_MAX: u8 = MASK_LOW;
const COLOR_CODE_LOW_MAX: u8 = MASK_TRIPLET;
const COLOR_OFFSET_HIGH: u8 = 100;
const COLOR_OFFSET_LOW: u8 = 40;
const HIGH_CODES_UPPER_BOUNDARY: u8 = COLOR_OFFSET_HIGH + COLOR_CODE_LOW_MAX;
const LOW_CODES_UPPER_BOUNDARY: u8 = COLOR_OFFSET_LOW + COLOR_CODE_LOW_MAX;
const MASK_HIGH: u8 = 0b1111_0000;
const MASK_LOW: u8 = 0b0000_1111;
const MASK_TRIPLET: u8 = MASK_LOW >> 1;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnsiColorCode(u8);

impl AnsiColorCode {
    /// Creates a new color code
    pub const fn new(number: u8) -> Self {
        Self(number)
    }

    pub const fn from_lower_byte_half(byte: u8) -> Self {
        Self::from_byte_half(byte & MASK_LOW)
    }

    pub const fn from_upper_byte_half(byte: u8) -> Self {
        Self::from_byte_half((byte & MASK_HIGH) >> MASK_BITS)
    }

    const fn from_byte_half(byte: u8) -> Self {
        match byte {
            value @ ..=COLOR_CODE_LOW_MAX => Self::new(value + COLOR_OFFSET_LOW),
            value @ ..=COLOR_CODE_HIGH_MAX => Self::new((value & MASK_TRIPLET) + COLOR_OFFSET_HIGH),
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for AnsiColorCode {
    type Error = Error;

    fn try_from(number: u8) -> Result<Self, Self::Error> {
        match number {
            number @ (COLOR_OFFSET_LOW..=LOW_CODES_UPPER_BOUNDARY
            | COLOR_OFFSET_HIGH..=HIGH_CODES_UPPER_BOUNDARY) => Ok(Self::new(number)),
            number => Err(Error::ValueOutOfBounds(number)),
        }
    }
}

impl Display for AnsiColorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{CHAR_START}{NUMBER_PREFIX}{}{NUMBER_SUFFIX} ", self.0)
    }
}

impl From<AnsiColorCode> for u8 {
    fn from(code: AnsiColorCode) -> Self {
        if code.0 < COLOR_OFFSET_HIGH {
            code.0 - COLOR_OFFSET_LOW
        } else {
            code.0 - COLOR_OFFSET_HIGH + COLOR_CODE_HIGH_BIT
        }
    }
}
