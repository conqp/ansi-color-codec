use crate::ansi_to_bytes::AnsiColorCodesToBytesIterator;
use crate::bytes_to_ansi::BytesAsAnsiColorsIterator;
use crate::color_code::AnsiColorCode;
use crate::color_code_pair::AnsiColorCodePair;
use crate::Error;
use std::iter::Flatten;
use threaded_map::{ThreadedMap, ThreadedMappable};

/// Gives u8 iterators the ability to en- / decode bytes to / from ANSI background colors
#[allow(clippy::module_name_repetitions)]
pub trait AnsiColorCodec
where
    Self::Encoder: Iterator<Item = AnsiColorCode>,
    Self::Parser: Iterator<Item = Result<AnsiColorCode, Error>>,
    Self::Decoder: Iterator<Item = Result<u8, Error>>,
{
    type Encoder;
    type Parser;
    type Decoder;

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
    fn encode(self) -> Self::Encoder;

    /// Parses ANSI color codes from a byte iterator
    fn parse(self) -> Self::Parser;

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
    fn decode(self) -> Self::Decoder;
}

impl<T> AnsiColorCodec for T
where
    T: Iterator<Item = u8>,
{
    type Encoder = Flatten<ThreadedMap<T, fn(u8) -> AnsiColorCodePair, AnsiColorCodePair>>;
    type Parser = BytesAsAnsiColorsIterator<T>;
    type Decoder = AnsiColorCodesToBytesIterator<BytesAsAnsiColorsIterator<T>>;

    fn encode(self) -> Self::Encoder {
        self.parallel_map(AnsiColorCodePair::from as fn(u8) -> AnsiColorCodePair, None)
            .flatten()
    }

    fn parse(self) -> Self::Parser {
        self.into()
    }

    fn decode(self) -> Self::Decoder {
        <Self as AnsiColorCodec>::parse(self).into()
    }
}
