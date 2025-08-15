use core::iter::FlatMap;

use crate::Error;
use crate::code_pair::CodePair;

/// Gives the ability to encode bytes to ANSI background colors.
pub trait Encoder {
    /// Type to encode bytes into ANSI color codes.
    type Encoder;
    /// Error type.
    type Error;

    /// Encode bytes into ANSI colors.
    fn encode(self) -> Self::Encoder;
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
