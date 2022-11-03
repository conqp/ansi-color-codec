mod triplets;
use triplets::Triplets;

pub fn encode(
    bytes: impl Iterator<Item = u8> + 'static,
    fill: char,
) -> impl Iterator<Item = String> {
    Triplets::from(to_bits(bytes)).map(move |triplet| encode_color(triplet, fill))
}

fn to_bits(bytes: impl Iterator<Item = u8>) -> impl Iterator<Item = bool> {
    bytes.flat_map(|byte| (0..8).map(move |offset| byte & (1 << offset) != 0))
}

fn encode_color(color: u8, fill: char) -> String {
    format!("\x1b[{}m{}", color + 40, fill)
}
