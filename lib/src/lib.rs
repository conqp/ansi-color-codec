//! Library to encode arbitrary byte streams as ANSI background colors.
#![no_std]

pub use self::codec::Codec;
pub use self::constants::RESET;
pub use self::decoder::Decoder;
pub use self::encoder::Encoder;
pub use self::error::Error;

mod code;
mod code_pair;
mod codec;
mod constants;
mod decoder;
mod encoder;
mod error;
mod pair_decoder;
mod parser;
