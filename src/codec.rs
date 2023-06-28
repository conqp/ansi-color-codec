use crate::ansi_to_bytes::AnsiColorCodesToBytesIterator;
use crate::bytes_to_ansi::BytesAsAnsiColorsIterator;
use crate::color_code::AnsiColorCode;
use crate::color_code_pair::AnsiColorCodePair;
use crate::Error;
use std::iter::Flatten;
use threaded_map::{ThreadedMap, ThreadedMappable};

/// Gives u8 iterators the ability to en- / decode bytes to / from ANSI background colors
#[allow(clippy::module_name_repetitions)]
pub trait AnsiColorCodec<E, P, D>
where
    E: Iterator<Item = AnsiColorCode>,
    P: Iterator<Item = Result<AnsiColorCode, Error>>,
    D: Iterator<Item = Result<u8, Error>>,
{
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
    fn encode(self) -> E;

    /// Parses ANSI color codes from a byte iterator
    fn parse(self) -> P;

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
    fn decode(self) -> D;
}

impl<T>
    AnsiColorCodec<
        Flatten<ThreadedMap<T, fn(u8) -> AnsiColorCodePair, AnsiColorCodePair>>,
        BytesAsAnsiColorsIterator<T>,
        AnsiColorCodesToBytesIterator<BytesAsAnsiColorsIterator<T>>,
    > for T
where
    T: Iterator<Item = u8>,
{
    fn encode(self) -> Flatten<ThreadedMap<T, fn(u8) -> AnsiColorCodePair, AnsiColorCodePair>> {
        self.parallel_map(AnsiColorCodePair::from as fn(u8) -> AnsiColorCodePair, None)
            .flatten()
    }

    fn parse(self) -> BytesAsAnsiColorsIterator<T> {
        self.into()
    }

    fn decode(self) -> AnsiColorCodesToBytesIterator<BytesAsAnsiColorsIterator<T>> {
        <Self as AnsiColorCodec<
            Flatten<ThreadedMap<T, fn(u8) -> AnsiColorCodePair, AnsiColorCodePair>>,
            BytesAsAnsiColorsIterator<T>,
            AnsiColorCodesToBytesIterator<BytesAsAnsiColorsIterator<T>>,
        >>::parse(self)
        .into()
    }
}
