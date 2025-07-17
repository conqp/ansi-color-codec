//! Library to encode arbitrary byte streams as ANSI background colors.
#![no_std]

pub use codec::Codec;
pub use constants::RESET;
pub use error::Error;

mod code;
mod code_pair;
mod codec;
mod constants;
mod error;
mod pair_decoder;
mod parser;
