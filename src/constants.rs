pub const MASK_LOW: u8 = 0b0000_1111;
pub const CODE_START: u8 = 0x1b;
pub const COLOR_OFFSET_HIGH: u8 = 100;
pub const COLOR_OFFSET_LOW: u8 = 40;
pub const MASK_TRIPLET: u8 = MASK_LOW >> 1;
pub const NUMBER_PREFIX: char = '[';
pub const NUMBER_SUFFIX: char = 'm';
/// Print this str to reset a color code sequence on the terminal
pub const RESET: &str = "\x1b[0m ";
