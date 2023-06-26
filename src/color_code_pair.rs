use crate::color_code::AnsiColorCode;
use crate::constants::MASK_BITS;
use std::array::IntoIter;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnsiColorCodePair {
    high: AnsiColorCode,
    low: AnsiColorCode,
}

impl From<u8> for AnsiColorCodePair {
    fn from(byte: u8) -> Self {
        Self {
            high: AnsiColorCode::from_upper_byte_half(byte),
            low: AnsiColorCode::from_lower_byte_half(byte),
        }
    }
}

impl From<(AnsiColorCode, AnsiColorCode)> for AnsiColorCodePair {
    fn from((high, low): (AnsiColorCode, AnsiColorCode)) -> Self {
        Self { high, low }
    }
}

impl From<AnsiColorCodePair> for u8 {
    fn from(value: AnsiColorCodePair) -> Self {
        (Self::from(value.high) << MASK_BITS) + Self::from(value.low)
    }
}

impl IntoIterator for AnsiColorCodePair {
    type Item = AnsiColorCode;
    type IntoIter = IntoIter<AnsiColorCode, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.high, self.low].into_iter()
    }
}
