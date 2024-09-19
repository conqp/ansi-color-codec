//! Encode bytes into ANSI color codes and decode ANSI colors into bytes.
mod code;
mod code_pair;
mod codec;
mod constants;
mod error;
mod pair_decoder;
mod parser;

pub use code::Code;
pub use codec::{Codec, Decoder, Encoder};
pub use constants::RESET;
pub use error::Error;
