use crate::constants::{CODE_START, MASK_LOW, NUMBER_PREFIX, NUMBER_SUFFIX};
use crate::error::Error;
use std::fmt::{Display, Formatter};

const CHAR_START: char = CODE_START as char;
const COLOR_CODE_HIGH_BIT: u8 = 0b1000;
const COLOR_CODE_LOW_MAX: u8 = MASK_TRIPLET;
const COLOR_OFFSET_HIGH: u8 = 100;
const COLOR_OFFSET_LOW: u8 = 40;
const HIGH_CODES_UPPER_BOUNDARY: u8 = COLOR_OFFSET_HIGH + COLOR_CODE_LOW_MAX;
const LOW_CODES_UPPER_BOUNDARY: u8 = COLOR_OFFSET_LOW + COLOR_CODE_LOW_MAX;
const MASK_TRIPLET: u8 = MASK_LOW >> 1;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Eq, PartialEq)]
pub struct AnsiColorCode {
    number: u8,
}

impl AnsiColorCode {
    /// Creates a new color code
    pub const fn new(number: u8) -> Self {
        Self { number }
    }

    pub const fn from_byte(byte: u8) -> Self {
        match byte {
            value @ ..=COLOR_CODE_LOW_MAX => Self::new(value + COLOR_OFFSET_LOW),
            value => Self::new((value & MASK_TRIPLET) + COLOR_OFFSET_HIGH),
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

impl TryFrom<u8> for AnsiColorCode {
    type Error = Error;

    fn try_from(number: u8) -> Result<Self, Self::Error> {
        match number {
            number @ (0..=LOW_CODES_UPPER_BOUNDARY
            | COLOR_OFFSET_HIGH..=HIGH_CODES_UPPER_BOUNDARY) => Ok(Self::new(number)),
            number => Err(Error::ValueOutOfBounds(number)),
        }
    }
}

impl Display for AnsiColorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let number = self.number;
        write!(f, "{CHAR_START}{NUMBER_PREFIX}{number}{NUMBER_SUFFIX} ")
    }
}
