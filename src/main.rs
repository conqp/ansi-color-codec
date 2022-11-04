use clap::Parser;
use color_code::{decode, encode};
use std::io::Read;
use std::io::{stdin, stdout, Write};

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long, name = "decode")]
    pub decode: bool,

    #[clap(short, long, name = "no-clear")]
    pub no_clear: bool,
}

fn main() {
    let args = Args::parse();

    if args.decode {
        do_decode()
    } else {
        do_encode(!args.no_clear)
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

fn do_encode(clear: bool) {
    for code in encode(stdin().bytes().map(|result| result.unwrap())) {
        print!("{}", code);
    }

    if clear {
        println!("\x1b[0m");
    }
}
