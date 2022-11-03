use color_code::encode;
use std::io::stdin;
use std::io::Read;

fn main() {
    println!(
        "{}\x1b[0m",
        encode(stdin().bytes().map(|result| result.unwrap()))
    );
}
