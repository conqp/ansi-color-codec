mod bitstream;

use bitstream::{BitsToBytes, BitsToBytesIterator, BytesToBits};
use std::iter::{FlatMap, Map};

mod color_code;
use color_code::{BytesToColorCodes, ColorCode, ColorCodeIterator, ToColor};

mod triplets;
use triplets::{Triplet, TripletIterator, Triplets};

type ColorsIterator<'a> =
    Map<TripletIterator<Box<dyn Iterator<Item = bool> + 'a>>, fn(Triplet) -> String>;
type BytesIterator<T> = BitsToBytesIterator<
    FlatMap<
        ColorCodeIterator<T>,
        Box<dyn Iterator<Item = bool>>,
        fn(ColorCode) -> Box<dyn Iterator<Item = bool>>,
    >,
>;
type TripletGenerator = fn(ColorCode) -> Box<dyn Iterator<Item = bool>>;

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
