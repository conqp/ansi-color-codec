pub const MASK_LOW: u8 = 0b0000_1111;
pub const CODE_START: u8 = 0x1b;
pub const NUMBER_PREFIX: char = '[';
pub const NUMBER_SUFFIX: char = 'm';
/// Print this str to reset a color code sequence on the terminal
pub const RESET: &str = "\x1b[0m ";
