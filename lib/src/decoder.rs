use crate::pair_decoder::PairDecoder;
use crate::parser::Parser;

/// Gives the ability to decode bytes from ANSI background colors.
pub trait Decoder {
    /// A type to parse color codes.
    type Parser;
    /// A type to decode color codes.
    type Decoder;

    /// Parse bytes into color codes.
    fn parse(self) -> Self::Parser;

    /// Decode color codes to bytes.
    fn decode(self) -> Self::Decoder;
}

impl<T> Decoder for T {
    type Parser = Parser<T>;
    type Decoder = PairDecoder<Parser<T>>;

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
