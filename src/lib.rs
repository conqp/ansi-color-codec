mod bitstream;

use bitstream::{BitsToBytes, BitsToBytesIterator, BytesToBits, BytesToBitsIterator};
use std::iter::{FlatMap, Map};

mod color_code;
use color_code::{BytesToColorCodes, ColorCode, ColorCodeIterator};

mod triplets;
use triplets::{ToColor, Triplet, TripletIterator, Triplets};

type ColorsIterator<T> = Map<TripletIterator<BytesToBitsIterator<T>>, fn(Triplet) -> String>;
type BytesIterator<T> = BitsToBytesIterator<
    FlatMap<
        ColorCodeIterator<T>,
        Box<dyn Iterator<Item = bool>>,
        fn(ColorCode) -> Box<dyn Iterator<Item = bool>>,
    >,
>;
type TripletGenerator = fn(ColorCode) -> Box<dyn Iterator<Item = bool>>;

pub trait ColorCodec<T>
where
    T: Iterator<Item = u8>,
{
    fn color_code(self) -> ColorsIterator<T>;
    fn color_decode(self) -> BytesIterator<T>;
}

impl<T> ColorCodec<T> for T
where
    T: Iterator<Item = u8>,
{
    fn color_code(self) -> ColorsIterator<T> {
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
