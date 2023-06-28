use crate::color_code::AnsiColorCode;
use crate::color_code_pair::AnsiColorCodePair;
use crate::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct AnsiColorCodesToBytesIterator<T>
where
    T: Iterator<Item = Result<AnsiColorCode, Error>>,
{
    codes: T,
}

impl<T> From<T> for AnsiColorCodesToBytesIterator<T>
where
    T: Iterator<Item = Result<AnsiColorCode, Error>>,
{
    fn from(codes: T) -> Self {
        Self { codes }
    }
}

impl<T> Iterator for AnsiColorCodesToBytesIterator<T>
where
    T: Iterator<Item = Result<AnsiColorCode, Error>>,
{
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.codes.next().map(|high| {
            high.and_then(|high| {
                self.codes.next().map_or_else(
                    || Err(Error::MissingSecondColorCodeBlock),
                    |low| match low {
                        Ok(low) => Ok(u8::from(AnsiColorCodePair::from([high, low]))),
                        Err(error) => Err(error),
                    },
                )
            })
        })
    }
}
