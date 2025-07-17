use alloc::string::ToString;
use alloc::vec::IntoIter;
use core::fmt::{self, Display, Formatter};

use crate::constants::{CODE_START, MASK_BITS, NUMBER_PREFIX, NUMBER_SUFFIX};
use crate::error::Error;

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

/// An ANSI color code segment, encoding a nibble.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Code(u8);

impl Code {
    /// Parse a [`Code`] from the lower half of a byte.
    #[must_use]
    pub const fn from_lower_nibble(byte: u8) -> Self {
        Self::from_nibble(byte & MASK_LOW)
    }

    /// Parse a [`Code`] from the upper half of a byte.
    #[must_use]
    pub const fn from_upper_nibble(byte: u8) -> Self {
        Self::from_nibble((byte & MASK_HIGH) >> MASK_BITS)
    }

    /// Return a half-byte sized value from the color code.
    #[must_use]
    pub const fn to_nibble(self) -> u8 {
        if self.0 < COLOR_OFFSET_HIGH {
            self.0 - COLOR_OFFSET_LOW
        } else {
            self.0 - COLOR_OFFSET_HIGH + COLOR_CODE_HIGH_BIT
        }
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
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{CHAR_START}{NUMBER_PREFIX}{}{NUMBER_SUFFIX} ", self.0)
    }
}

impl IntoIterator for Code {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_string().into_bytes().into_iter()
    }
}

impl TryFrom<u8> for Code {
    type Error = Error;

    /// Attempt to crate a [`Code`] from an unsigned byte.
    ///
    /// # Errors
    ///
    /// * [`Error::ValueOutOfBounds`] - if the number is not within valid bounds
    fn try_from(number: u8) -> Result<Self, Self::Error> {
        match number {
            number @ (COLOR_OFFSET_LOW..=LOW_CODES_UPPER_BOUNDARY
            | COLOR_OFFSET_HIGH..=HIGH_CODES_UPPER_BOUNDARY) => Ok(Self(number)),
            number => Err(Error::ValueOutOfBounds(number)),
        }
    }
}
