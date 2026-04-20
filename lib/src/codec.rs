use crate::Error;
use crate::code::Code;
use crate::decoder::Decoder;
use crate::encoder::Encoder;

/// Gives the ability to en- / decode bytes to / from ANSI background colors.
pub trait Codec {
    /// Encode bytes into ANSI colors.
    fn encode(self) -> impl Iterator<Item = Code>;

    /// Decode ANSI color codes into bytes.
    fn decode(self) -> impl Iterator<Item = Result<u8, Error>>;
}

impl<T> Codec for T
where
    T: Encoder + Decoder,
{
    fn encode(self) -> impl Iterator<Item = Code> {
        <Self as Encoder>::encode(self)
    }

    fn decode(self) -> impl Iterator<Item = Result<u8, Error>> {
        <Self as Decoder>::decode(self)
    }
}
