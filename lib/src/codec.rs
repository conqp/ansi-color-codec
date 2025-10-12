pub use crate::decoder::Decoder;
pub use crate::encoder::Encoder;

/// Gives the ability to en- / decode bytes to / from ANSI background colors.
pub trait Codec: Encoder + Decoder {
    /// Encode bytes into ANSI colors.
    fn encode(self) -> <Self as Encoder>::Encoder;

    /// Decode ANSI color codes into bytes.
    fn decode(self) -> <Self as Decoder>::Decoder;
}

impl<T> Codec for T
where
    T: Encoder + Decoder,
{
    fn encode(self) -> <Self as Encoder>::Encoder {
        <Self as Encoder>::encode(self)
    }

    fn decode(self) -> <Self as Decoder>::Decoder {
        <Self as Decoder>::decode(self)
    }
}
