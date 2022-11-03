use color_code::encode;
use std::io::stdin;
use std::io::Read;

fn main() {
    for code in encode(stdin().bytes().map(|result| result.unwrap())) {
        print!("{}", code);
    }

    println!("\x1b[0m");
}
