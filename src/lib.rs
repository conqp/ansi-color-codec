mod triplets;

use triplets::Triplets;

pub fn encode(bytes: impl Iterator<Item = u8> + 'static) -> String {
    encode_with_fill(bytes, ' ')
}

pub fn encode_with_fill(bytes: impl Iterator<Item = u8> + 'static, fill: char) -> String {
    Triplets::from(to_bits(bytes))
        .map(|triplet| encode_color(triplet, fill))
        .collect()
}

fn to_bits(bytes: impl Iterator<Item = u8>) -> impl Iterator<Item = bool> {
    bytes.flat_map(|byte| (0..8).map(move |offset| byte & (1 << offset) != 0))
}

fn encode_color(color: u8, fill: char) -> String {
    format!("\x1b[{}m{}", color + 40, fill)
}
