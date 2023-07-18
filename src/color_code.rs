use crate::constants::{CODE_START, MASK_BITS, NUMBER_PREFIX, NUMBER_SUFFIX};
use crate::error::Error;
use std::fmt::{Display, Formatter};

const CHAR_START: char = CODE_START as char;
const COLOR_CODE_HIGH_BIT: u8 = 0b1000;
const COLOR_CODE_HIGH_MAX: u8 = MASK_LOW;
const COLOR_CODE_HIGH_MIN: u8 = COLOR_CODE_LOW_MAX + 1;
const COLOR_CODE_LOW_MAX: u8 = MASK_TRIPLET;
const COLOR_OFFSET_HIGH: u8 = 100;
const COLOR_OFFSET_LOW: u8 = 40;
const HIGH_CODES_UPPER_BOUNDARY: u8 = COLOR_OFFSET_HIGH + COLOR_CODE_LOW_MAX;
const LOW_CODES_UPPER_BOUNDARY: u8 = COLOR_OFFSET_LOW + COLOR_CODE_LOW_MAX;
const MASK_HIGH: u8 = MASK_LOW << MASK_BITS;
const MASK_LOW: u8 = 0b0000_1111;
const MASK_TRIPLET: u8 = MASK_LOW >> 1;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnsiColorCode(u8);

impl AnsiColorCode {
    /// Parses an [`AnsiColorCode`] from the lower half of a byte
    ///
    /// # Arguments
    /// * `byte` - The byte to parse from
    #[must_use]
    pub const fn from_lower_nibble(byte: u8) -> Self {
        Self::from_nibble(byte & MASK_LOW)
    }

    /// Parses an [`AnsiColorCode`] from the upper half of a byte
    ///
    /// # Arguments
    /// * `byte` - The byte to parse from
    #[must_use]
    pub const fn from_upper_nibble(byte: u8) -> Self {
        Self::from_nibble((byte & MASK_HIGH) >> MASK_BITS)
    }

    const fn from_nibble(byte: u8) -> Self {
        match byte {
            value @ 0..=COLOR_CODE_LOW_MAX => Self(value + COLOR_OFFSET_LOW),
            value @ COLOR_CODE_HIGH_MIN..=COLOR_CODE_HIGH_MAX => {
                Self((value & MASK_TRIPLET) + COLOR_OFFSET_HIGH)
            }
            _ => unreachable!(),
        }
    }

    /// Returns a half-byte sized value from the color code
    #[must_use]
    pub const fn to_nibble(self) -> u8 {
        if self.0 < COLOR_OFFSET_HIGH {
            self.0 - COLOR_OFFSET_LOW
        } else {
            self.0 - COLOR_OFFSET_HIGH + COLOR_CODE_HIGH_BIT
        }
    }
}

impl Display for AnsiColorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{CHAR_START}{NUMBER_PREFIX}{}{NUMBER_SUFFIX} ", self.0)
    }
}

impl TryFrom<u8> for AnsiColorCode {
    type Error = Error;

    /// Attempts to crate an [`AnsiColorCode`] from an unsigned byte
    ///
    /// # Arguments
    /// * `number` - The unsigned byte to parse
    ///
    /// # Errors
    /// * [`Error::ValueOutOfBounds`] - if the number is not within valid bounds
    fn try_from(number: u8) -> Result<Self, Self::Error> {
        match number {
            number @ (COLOR_OFFSET_LOW..=LOW_CODES_UPPER_BOUNDARY
            | COLOR_OFFSET_HIGH..=HIGH_CODES_UPPER_BOUNDARY) => Ok(Self(number)),
            number => Err(Error::ValueOutOfBounds(number)),
        }
    }
}
