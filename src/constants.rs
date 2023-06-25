pub const MASK_LOW: u8 = 0b0000_1111;
pub const MASK_HIGH: u8 = 0b1111_0000;
pub const MASK_BITS: u8 = 4;
pub const MASK_TRIPLET: u8 = MASK_LOW >> 1;
pub const COLOR_OFFSET_LOW: u8 = 40;
pub const COLOR_OFFSET_HIGH: u8 = 100;
pub const COLOR_CODE_LOW_MAX: u8 = MASK_TRIPLET;
pub const COLOR_CODE_MAX: u8 = MASK_LOW;
pub const COLOR_CODE_HIGH_BIT: u8 = 0b1000;
pub const MAX_DIGITS: u8 = 3;
pub const CODE_START: u8 = 0x1b;
pub const NUMBER_PREFIX: char = '[';
pub const NUMBER_SUFFIX: char = 'm';
pub const SPACE: char = ' ';
/// Print this str to reset a color code sequence on the terminal
pub const RESET: &str = "\x1b[0m ";
