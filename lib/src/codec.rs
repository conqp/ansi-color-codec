pub use crate::decoder::Decoder;
pub use crate::encoder::Encoder;

/// Gives the ability to en- / decode bytes to / from ANSI background colors.
pub trait Codec {
    /// Type to encode bytes into ANSI color codes.
    type Encoder;
    /// A type to decode color codes.
    type Decoder;

    /// Encode bytes into ANSI colors.
    fn encode(self) -> Self::Encoder;

    /// Decode ANSI color codes into bytes.
    fn decode(self) -> Self::Decoder;
}

impl<T> Codec for T
where
    T: Encoder + Decoder,
{
    type Encoder = <Self as Encoder>::Encoder;
    type Decoder = <Self as Decoder>::Decoder;

    fn encode(self) -> Self::Encoder {
        <Self as Encoder>::encode(self)
    }

    fn decode(self) -> Self::Decoder {
        <Self as Decoder>::decode(self)
    }
}
