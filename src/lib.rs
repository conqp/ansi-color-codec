pub fn encode(bytes: &[u8]) -> String {
    to_triplets(&to_bits(bytes))
        .iter()
        .map(|triplet| encode_color(*triplet))
        .collect()
}

fn to_bits(bytes: &[u8]) -> Vec<bool> {
    bytes
        .iter()
        .flat_map(|byte| (0..8).map(move |offset| byte & (1 << offset) != 0))
        .collect()
}

fn to_triplets(bits: &[bool]) -> Vec<u8> {
    let mut triplets = Vec::new();
    let mut triplet = 0;

    for (index, bit) in bits.iter().enumerate() {
        triplet += (*bit as u8) << (index % 3);

        if index % 3 == 2 {
            triplets.push(triplet);
            triplet = 0;
        }
    }

    if triplet != 0 {
        triplets.push(triplet);
    }

    triplets
}

fn encode_color(color: u8) -> String {
    format!("\x1b[{}m ", color + 40)
}
