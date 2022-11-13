mod bitstream;

use bitstream::{BitsToBytes, BytesToBits};
use std::iter::{FlatMap, Map};

mod color_code;
use color_code::BytesToColorCodes;

mod triplets;
use crate::bitstream::{BitsToBytesIterator, BytesToBitsIterator};
use crate::color_code::{ColorCode, ColorCodeIterator};
use crate::triplets::{Triplet, TripletIterator};
use triplets::{ToColor, Triplets};

type ColorsIterator<T> = Map<TripletIterator<BytesToBitsIterator<T>>, fn(Triplet) -> String>;
type BytesIterator<T> = BitsToBytesIterator<
    FlatMap<
        ColorCodeIterator<T>,
        Box<dyn Iterator<Item = bool>>,
        fn(ColorCode) -> Box<dyn Iterator<Item = bool>>,
    >,
>;

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
            .flat_map((|code| code.triplets()) as fn(ColorCode) -> Box<dyn Iterator<Item = bool>>)
            .bytes()
    }
}
