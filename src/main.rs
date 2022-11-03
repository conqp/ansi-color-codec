use color_code::encode;
use std::io::stdin;
use std::io::Read;

fn main() {
    let bytes: Vec<u8> = stdin().bytes().map(|result| result.unwrap()).collect();
    let result = encode(&bytes);
    println!("{}\x1b[0m", result);
}
