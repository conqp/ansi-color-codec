use clap::Parser;
use color_code::{ColorCodec, CLEARING_CODE};
use ctrlc::set_handler;
use std::io::{stdin, stdout, Read, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

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
    let bytes = stdin_while_running(running.clone());

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
    for byte in bytes.color_decode() {
        if stdout().write(&[byte]).is_err() {
            return;
        }
    }

    stdout().flush().expect("Could not flush STDOUT");
}

fn encode(bytes: impl Iterator<Item = u8>, clear: bool) {
    for code in bytes.color_code() {
        if stdout().write_all(code.to_string().as_bytes()).is_err() {
            return;
        }
    }

    if clear {
        stdout()
            .write_all(CLEARING_CODE.as_bytes())
            .expect("Could not write clearing code");
    }
}

fn stdin_while_running(running: Arc<AtomicBool>) -> impl Iterator<Item = u8> {
    stdin()
        .bytes()
        .take_while(move |byte| byte.is_ok() && running.load(Ordering::SeqCst))
        .map(|result| result.unwrap())
}
