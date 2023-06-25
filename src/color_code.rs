use crate::constants::{
    CODE_START, COLOR_CODE_HIGH_BIT, COLOR_CODE_LOW_MAX, COLOR_CODE_MAX, COLOR_OFFSET_HIGH,
    COLOR_OFFSET_LOW, MASK_TRIPLET, NUMBER_PREFIX, NUMBER_SUFFIX, SPACE,
};
use crate::error::Error;
use std::fmt::{Display, Formatter};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Eq, PartialEq)]
pub struct AnsiColorCode {
    number: u8,
}

impl AnsiColorCode {
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

impl TryFrom<u8> for AnsiColorCode {
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

impl Display for AnsiColorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            CODE_START as char, NUMBER_PREFIX, self.number, NUMBER_SUFFIX, SPACE
        )
    }
}
