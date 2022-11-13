mod bitstream;

use bitstream::{BitsToBytes, BitsToBytesIterator, BytesToBits, TripletIterator, Triplets};
use std::array::IntoIter;
use std::iter::{FlatMap, Map};

mod color_code;
use color_code::{BytesToColorCodes, ColorCode, ColorCodeIterator, ToColor};

type ColorsIterator<'a> =
    Map<TripletIterator<Box<dyn Iterator<Item = bool> + 'a>>, fn(u8) -> String>;
type TripletGenerator = fn(ColorCode) -> IntoIter<bool, 3>;
type BytesIterator<T> =
    BitsToBytesIterator<FlatMap<ColorCodeIterator<T>, IntoIter<bool, 3>, TripletGenerator>>;

pub trait ColorCodec<'a, T>
where
    T: Iterator<Item = u8>,
{
    fn color_code(self) -> ColorsIterator<'a>;
    fn color_decode(self) -> BytesIterator<T>;
}

impl<'a, T> ColorCodec<'a, T> for T
where
    T: Iterator<Item = u8> + 'a,
{
    fn color_code(self) -> ColorsIterator<'a> {
        self.bits()
            .triplets()
            .map(move |triplet| triplet.to_color())
    }

    fn color_decode(self) -> BytesIterator<T> {
        self.codes()
            .flat_map((|code| code.triplets()) as TripletGenerator)
            .bytes()
    }
}
