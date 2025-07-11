use std::array::IntoIter;

use crate::code::Code;
use crate::constants::MASK_BITS;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CodePair(u8);

impl From<u8> for CodePair {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

impl From<[Code; 2]> for CodePair {
    fn from([high, low]: [Code; 2]) -> Self {
        Self((high.to_nibble() << MASK_BITS) + low.to_nibble())
    }
}

impl From<CodePair> for u8 {
    fn from(value: CodePair) -> Self {
        value.0
    }
}

impl IntoIterator for CodePair {
    type Item = Code;
    type IntoIter = IntoIter<Code, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [
            Code::from_upper_nibble(self.0),
            Code::from_lower_nibble(self.0),
        ]
        .into_iter()
    }
}
