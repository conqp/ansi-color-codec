//! Encode bytes into ANSI color codes and decode ANSI colors into bytes.

use std::io::{stdin, stdout, Read, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use codec::Codec;
use constants::RESET;
use error::Error;

use clap::Parser;
use ctrlc::set_handler;

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long, name = "decode")]
    decode: bool,

    #[clap(short, long, name = "no-clear")]
    no_clear: bool,
}
mod code;
mod code_pair;
mod codec;
mod constants;
mod error;
mod pair_decoder;
mod parser;

fn main() {
    let args = Args::parse();
    let running = Arc::new(AtomicBool::new(true));
    let bytes = stream_stdin(running.clone());
    let dst = stdout().lock();

    set_handler(move || {
        running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    if args.decode {
        decode(bytes, dst);
    } else {
        encode(bytes, dst, !args.no_clear);
    }
}

fn decode(bytes: impl Iterator<Item = u8>, mut dst: impl Write) {
    bytes
        .decode()
        .enumerate()
        .filter_map(|(index, result)| {
            result
                .inspect_err(|error| eprintln!("{error} at {index}"))
                .ok()
        })
        .map_while(|byte| dst.write_all(&[byte]).ok())
        .for_each(drop);

    dst.flush().unwrap_or_else(drop); // Ignore write errors here.
}

fn encode(bytes: impl Iterator<Item = u8>, mut dst: impl Write, clear: bool) {
    bytes
        .encode()
        .map_while(|code| write!(dst, "{code}").ok())
        .for_each(drop);

    if clear {
        write!(dst, "{RESET}").unwrap_or_else(drop); // Ignore write errors here.
    }
}

fn stream_stdin(running: Arc<AtomicBool>) -> impl Iterator<Item = u8> {
    stdin()
        .lock()
        .bytes()
        .take_while(move |_| running.load(Ordering::SeqCst))
        .map_while(Result::ok)
}
