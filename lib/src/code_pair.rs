use core::array::IntoIter;

use crate::code::Code;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CodePair(u8);

impl From<u8> for CodePair {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

impl From<[Code; 2]> for CodePair {
    fn from([high, low]: [Code; 2]) -> Self {
        Self(high.to_high_nibble() | low.to_low_nibble())
    }
}

impl From<CodePair> for u8 {
    fn from(value: CodePair) -> Self {
        value.0
    }
}

impl IntoIterator for CodePair {
    type Item = Code;
    type IntoIter = IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        Code::from_byte(self.0).into_iter()
    }
}
