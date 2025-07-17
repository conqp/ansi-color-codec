use core::iter::FlatMap;

use crate::Error;
use crate::code::Code;
use crate::code_pair::CodePair;
use crate::pair_decoder::PairDecoder;
use crate::parser::Parser;

/// Gives the ability to en- / decode bytes to / from ANSI background colors.
pub trait Codec: Encoder + Decoder + Sized {
    /// Encode bytes into ANSI colors.
    fn encode(self) -> <Self as Encoder>::Encoder {
        <Self as Encoder>::encode(self)
    }

    /// Decode ANSI color codes into bytes.
    fn decode(self) -> <Self as Decoder>::Decoder {
        <Self as Decoder>::decode(self)
    }
}

/// Gives the ability to encode bytes to ANSI background colors.
pub trait Encoder
where
    Self::Encoder: Iterator<Item = Code>,
{
    /// Type to encode bytes into ANSI color codes.
    type Encoder;
    /// Error type.
    type Error;

    /// Encode bytes into ANSI colors.
    fn encode(self) -> Self::Encoder;
}

/// Gives the ability to decode bytes from ANSI background colors.
pub trait Decoder
where
    Self::Parser: Iterator<Item = Result<Code, Self::Error>>,
    Self::Decoder: Iterator<Item = Result<u8, Self::Error>>,
{
    /// A type to parse color codes.
    type Parser;
    /// A type to decode color codes.
    type Decoder;
    /// Error type.
    type Error;

    /// Parse bytes into color codes.
    fn parse(self) -> Self::Parser;

    /// Decode color codes to bytes.
    fn decode(self) -> Self::Decoder;
}

impl<T> Encoder for T
where
    T: Iterator<Item = u8>,
{
    type Encoder = FlatMap<T, CodePair, fn(u8) -> CodePair>;
    type Error = Error;

    /// Return an iterator that encodes all bytes as ANSI background colors.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_color_codec::Codec;
    ///
    /// let text = String::from("Hello world.");
    /// let reference: Vec<u8> = vec![
    ///     27, 91, 52, 52, 109, 32, 27, 91, 49, 48, 48, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91,
    ///     52, 53, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 52, 109, 32, 27, 91, 52, 54,
    ///     109, 32, 27, 91, 49, 48, 52, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 55, 109,
    ///     32, 27, 91, 52, 50, 109, 32, 27, 91, 52, 48, 109, 32, 27, 91, 52, 55, 109, 32, 27, 91,
    ///     52, 55, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 55, 109, 32, 27, 91, 52, 55,
    ///     109, 32, 27, 91, 52, 50, 109, 32, 27, 91, 52, 54, 109, 32, 27, 91, 49, 48, 52, 109, 32,
    ///     27, 91, 52, 54, 109, 32, 27, 91, 52, 52, 109, 32, 27, 91, 52, 50, 109, 32, 27, 91, 49,
    ///     48, 54, 109, 32,
    /// ];
    /// let code: Vec<u8> = text
    ///     .bytes()
    ///     .encode()
    ///     .flat_map(|color| color.to_string().into_bytes())
    ///     .collect();
    /// assert_eq!(code, reference);
    /// ```
    fn encode(self) -> Self::Encoder {
        self.flat_map(CodePair::from)
    }
}

impl<T> Decoder for T
where
    T: Iterator<Item = u8>,
{
    type Parser = Parser<T>;
    type Decoder = PairDecoder<Parser<T>>;
    type Error = Error;

    /// Parse ANSI color codes from a byte iterator.
    fn parse(self) -> Self::Parser {
        self.into()
    }

    /// Return an iterator that decodes all bytes interpreted as a sequence of ANSI background
    /// colors to raw bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_color_codec::Codec;
    ///
    /// let text = String::from("Hello world.");
    /// let code: String = text
    ///     .bytes()
    ///     .encode()
    ///     .map(|color| color.to_string())
    ///     .collect();
    /// let decoded: String = code
    ///     .bytes()
    ///     .decode()
    ///     .filter_map(|result| result.map(|byte| byte as char).ok())
    ///     .collect();
    /// assert_eq!(text, decoded);
    /// ```
    fn decode(self) -> Self::Decoder {
        <Self as Decoder>::parse(self).into()
    }
}

impl<T> Codec for T where T: Encoder + Decoder + Sized {}
