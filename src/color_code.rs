use crate::constants::{
    CODE_START, COLOR_OFFSET_HIGH, COLOR_OFFSET_LOW, MASK_TRIPLET, NUMBER_PREFIX, NUMBER_SUFFIX,
};
use crate::error::Error;
use std::fmt::{Display, Formatter};

const CHAR_START: char = CODE_START as char;
const COLOR_CODE_HIGH_BIT: u8 = 0b1000;
const COLOR_CODE_LOW_MAX: u8 = MASK_TRIPLET;
const HIGH_CODES_UPPER_BOUNDARY: u8 = COLOR_OFFSET_HIGH + COLOR_CODE_LOW_MAX;
const LOW_CODES_UPPER_BOUNDARY: u8 = COLOR_OFFSET_LOW + COLOR_CODE_LOW_MAX;

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
            number => Err(Error::InvalidColorCode(number)),
        }
    }
}

impl Display for AnsiColorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let number = self.number;
        write!(f, "{CHAR_START}{NUMBER_PREFIX}{number}{NUMBER_SUFFIX} ")
    }
}
