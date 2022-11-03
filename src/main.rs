use clap::Parser;
use color_code::{decode, encode};
use std::io::Read;
use std::io::{stdin, stdout, Write};

#[derive(Parser)]
#[clap(about, author, version)]
pub struct Args {
    #[clap(short, long, name = "decode")]
    pub decode: bool,

    #[clap(short, long, name = "no-clear")]
    pub no_clear: bool,

    #[clap(short, long, name = "fill", value_parser, default_value_t = ' ')]
    pub fill: char,
}

fn main() {
    let args = Args::parse();

    if args.decode {
        do_decode()
    } else {
        do_encode(args.fill, !args.no_clear)
    }
}

fn do_decode() {
    for byte in decode(stdin().bytes().map(|result| result.unwrap())) {
        if stdout().write(&[byte]).is_err() {
            return;
        }
    }

    stdout().flush().expect("Could not flush STDOUT");
}

fn do_encode(fill: char, clear: bool) {
    for code in encode(stdin().bytes().map(|result| result.unwrap()), fill) {
        print!("{}", code);
    }

    if clear {
        println!("\x1b[0m");
    }
}
