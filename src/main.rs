use clap::Parser;
use color_code::encode;
use std::io::stdin;
use std::io::Read;

#[derive(Parser)]
#[clap(about, author, version)]
pub struct Args {
    #[clap(short, long, name = "no-clear")]
    pub no_clear: bool,

    #[clap(short, long, name = "fill", value_parser, default_value_t = ' ')]
    pub fill: char,
}

fn main() {
    let args = Args::parse();

    for code in encode(stdin().bytes().map(|result| result.unwrap()), args.fill) {
        print!("{}", code);
    }

    if !args.no_clear {
        println!("\x1b[0m");
    }
}
