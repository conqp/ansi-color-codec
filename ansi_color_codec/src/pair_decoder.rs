use crate::code::Code;
use crate::code_pair::CodePair;
use crate::error::Error;

/// Decodes color code pairs into values of u8
#[derive(Debug, Eq, PartialEq)]
pub struct PairDecoder<T>
where
    T: Iterator<Item = Result<Code, Error>>,
{
    codes: T,
}

impl<T> From<T> for PairDecoder<T>
where
    T: Iterator<Item = Result<Code, Error>>,
{
    fn from(codes: T) -> Self {
        Self { codes }
    }
}

impl<T> Iterator for PairDecoder<T>
where
    T: Iterator<Item = Result<Code, Error>>,
{
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.codes.next().map(|high| {
            high.and_then(|high| {
                self.codes.next().map_or_else(
                    || Err(Error::MissingSecondColorCodeBlock),
                    |low| low.map(|low| u8::from(CodePair::from([high, low]))),
                )
            })
        })
    }
}
