use crate::Error;
use crate::pair_decoder::PairDecoder;
use crate::parser::Parser;

/// Gives the ability to decode bytes from ANSI background colors.
pub trait Decoder {
    /// Decode color codes to bytes.
    fn decode(self) -> impl Iterator<Item = Result<u8, Error>>;
}

impl<T> Decoder for T
where
    T: Iterator<Item = u8>,
{
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
    fn decode(self) -> impl Iterator<Item = Result<u8, Error>> {
        PairDecoder::from(Parser::from(self))
    }
}
