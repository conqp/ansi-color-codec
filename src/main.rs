use ansi_color_codec::{ColorCodec, RESET};
use clap::Parser;
use ctrlc::set_handler;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

const STDOUT_WRITE_ERR: &str = "Could not write bytes to STDOUT";

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
    let running = Arc::new(AtomicBool::new(true));
    let bytes = stream_stdin(running.clone());

    set_handler(move || {
        running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    if args.decode {
        decode(bytes)
    } else {
        encode(bytes, !args.no_clear)
    }
}

fn decode(bytes: impl Iterator<Item = u8>) {
    for result in bytes.ansi_color_decode() {
        match result {
            Ok(byte) => {
                stdout().write_all(&[byte]).expect(STDOUT_WRITE_ERR);
            }
            Err(msg) => {
                eprintln!("{}", msg);
                exit(1);
            }
        }
    }

    stdout().flush().expect("Could not flush STDOUT")
}

fn encode(bytes: impl Iterator<Item = u8>, clear: bool) {
    for code in bytes.ansi_color_encode() {
        stdout()
            .write_all(code.to_string().as_bytes())
            .expect(STDOUT_WRITE_ERR);
    }

    if clear {
        println!("{}", RESET);
    }
}

fn stream_stdin(running: Arc<AtomicBool>) -> impl Iterator<Item = u8> {
    stdin()
        .bytes()
        .take_while(move |byte| byte.is_ok() && running.load(Ordering::SeqCst))
        .map(|byte| byte.unwrap())
}
