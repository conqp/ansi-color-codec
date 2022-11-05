mod bitstream;
use bitstream::{BitsToBytes, BytesToBits};

mod color_code;
use color_code::BytesToColorCodes;

mod triplets;
use triplets::{ToColor, Triplets};

pub fn encode(bytes: impl Iterator<Item = u8>) -> impl Iterator<Item = String> {
    bytes
        .bits()
        .triplets()
        .map(move |triplet| triplet.to_color())
}

pub fn decode(bytes: impl Iterator<Item = u8>) -> impl Iterator<Item = u8> {
    bytes.codes().flat_map(move |code| code.triplets()).bytes()
}
