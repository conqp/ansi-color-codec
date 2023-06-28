use crate::ansi_to_bytes::AnsiColorCodesToBytesIterator;
use crate::bytes_to_ansi::BytesAsAnsiColorsIterator;
use crate::color_code::AnsiColorCode;
use crate::color_code_pair::AnsiColorCodePair;
use crate::Error;
use std::iter::FlatMap;

/// Gives u8 iterators the ability to en- / decode bytes to / from ANSI background colors
#[allow(clippy::module_name_repetitions)]
pub trait AnsiColorCodec: AnsiColorEncoder + AnsiColorDecoder + Sized {
    fn encode(self) -> <Self as AnsiColorEncoder>::Encoder {
        <Self as AnsiColorEncoder>::encode(self)
    }

    fn parse(self) -> <Self as AnsiColorDecoder>::Parser {
        <Self as AnsiColorDecoder>::parse(self)
    }

    fn decode(self) -> <Self as AnsiColorDecoder>::Decoder {
        <Self as AnsiColorDecoder>::decode(self)
    }
}

/// Gives u8 iterators the ability to encode bytes to ANSI background colors
pub trait AnsiColorEncoder
where
    Self::Encoder: Iterator<Item = AnsiColorCode>,
{
    type Encoder;
    type Error;

    fn encode(self) -> Self::Encoder;
}

/// Gives u8 iterators the ability to decode bytes from ANSI background colors
pub trait AnsiColorDecoder
where
    Self::Parser: Iterator<Item = Result<AnsiColorCode, Self::Error>>,
    Self::Decoder: Iterator<Item = Result<u8, Self::Error>>,
{
    type Parser;
    type Decoder;
    type Error;

    fn parse(self) -> Self::Parser;
    fn decode(self) -> Self::Decoder;
}

impl<T> AnsiColorEncoder for T
where
    T: Iterator<Item = u8>,
{
    type Encoder = FlatMap<T, AnsiColorCodePair, fn(u8) -> AnsiColorCodePair>;
    type Error = Error;

    /// Returns an iterator that encodes all bytes as ANSI background colors
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_color_codec::AnsiColorCodec;
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
    ///     .map(|color| color.to_string())
    ///     .collect::<String>()
    ///     .bytes()
    ///     .collect();
    /// assert_eq!(code, reference);
    /// ```
    fn encode(self) -> Self::Encoder {
        self.flat_map(AnsiColorCodePair::from)
    }
}

impl<T> AnsiColorDecoder for T
where
    T: Iterator<Item = u8>,
{
    type Parser = BytesAsAnsiColorsIterator<T>;
    type Decoder = AnsiColorCodesToBytesIterator<BytesAsAnsiColorsIterator<T>>;
    type Error = Error;

    /// Parses ANSI color codes from a byte iterator
    fn parse(self) -> Self::Parser {
        self.into()
    }

    /// Returns an iterator that decodes all bytes interpreted as a sequence of ANSI background
    /// colors to raw bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_color_codec::AnsiColorCodec;
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
        <Self as AnsiColorDecoder>::parse(self).into()
    }
}

impl<T> AnsiColorCodec for T where T: AnsiColorEncoder + AnsiColorDecoder + Sized {}
