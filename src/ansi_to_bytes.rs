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
        match self.codes.next() {
            Some(high) => match high {
                Ok(high) => self.codes.next().map_or_else(
                    || Some(Err(Error::MissingSecondColorCodeBlock)),
                    |low| match low {
                        Ok(low) => Some(Ok(u8::from(AnsiColorCodePair::from([high, low])))),
                        Err(error) => Some(Err(error)),
                    },
                ),
                Err(error) => Some(Err(error)),
            },
            None => None,
        }
    }
}
