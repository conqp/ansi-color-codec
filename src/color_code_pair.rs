use crate::color_code::AnsiColorCode;
use crate::constants::MASK_BITS;
use std::array::IntoIter;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnsiColorCodePair(u8);

impl From<u8> for AnsiColorCodePair {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

impl From<(AnsiColorCode, AnsiColorCode)> for AnsiColorCodePair {
    fn from((high, low): (AnsiColorCode, AnsiColorCode)) -> Self {
        Self((u8::from(high) << MASK_BITS) + u8::from(low))
    }
}

impl From<AnsiColorCodePair> for u8 {
    fn from(value: AnsiColorCodePair) -> Self {
        value.0
    }
}

impl IntoIterator for AnsiColorCodePair {
    type Item = AnsiColorCode;
    type IntoIter = IntoIter<AnsiColorCode, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [
            AnsiColorCode::from_upper_byte_half(self.0),
            AnsiColorCode::from_lower_byte_half(self.0),
        ]
        .into_iter()
    }
}
