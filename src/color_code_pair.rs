use crate::color_code::AnsiColorCode;
use crate::constants::{COLOR_OFFSET_HIGH, COLOR_OFFSET_LOW, MASK_TRIPLET};
use std::array::IntoIter;

const MASK_BITS: u8 = 4;
const MASK_HIGH: u8 = 0b1111_0000;

#[allow(clippy::module_name_repetitions)]
pub struct AnsiColorCodePair {
    high: AnsiColorCode,
    low: AnsiColorCode,
}

impl From<u8> for AnsiColorCodePair {
    fn from(value: u8) -> Self {
        Self {
            high: AnsiColorCode::new(((value & MASK_HIGH) >> MASK_BITS) + COLOR_OFFSET_LOW),
            low: AnsiColorCode::new((value & MASK_TRIPLET) + COLOR_OFFSET_HIGH),
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
        (value.high.normalized() << MASK_BITS) + value.low.normalized()
    }
}

impl IntoIterator for AnsiColorCodePair {
    type Item = AnsiColorCode;
    type IntoIter = IntoIter<AnsiColorCode, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.high, self.low].into_iter()
    }
}
