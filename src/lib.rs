mod bits;
use bits::BytesToBits;

mod bytes;
use bytes::BitsToBytes;

mod code;
use code::ToCodes;

mod triplets;
use triplets::{ToColor, Triplets};

pub fn encode(bytes: impl Iterator<Item = u8> + 'static) -> impl Iterator<Item = String> {
    bytes
        .bits()
        .triplets()
        .map(move |triplet| triplet.to_color())
}

pub fn decode(bytes: impl Iterator<Item = u8> + 'static) -> impl Iterator<Item = u8> {
    bytes
        .codes()
        .filter(|code| code.number() != 0)
        .flat_map(move |code| code.triplet().into_iter())
        .bytes()
}
