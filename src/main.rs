use ansi_color_codec::ColorCodec;
use clap::Parser;
use ctrlc::set_handler;
use std::io::{stdin, stdout, Read, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

const STDOUT_WRITE_ERR: &str = "Could not write bytes to STDOUT";
const UTF_16_BOM: [u8; 3] = [239, 187, 191];

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long, name = "decode")]
    pub decode: bool,

    #[clap(short, long, name = "no-clear")]
    pub no_clear: bool,

    #[clap(short, long, name = "skip-bom")]
    pub skip_bom: bool,
}

fn main() {
    let args = Args::parse();
    let running = Arc::new(AtomicBool::new(true));
    let bytes = stream_stdin(running.clone(), args.skip_bom);

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
    for byte in bytes.ansi_color_decode() {
        stdout().write_all(&[byte]).expect(STDOUT_WRITE_ERR);
    }

    stdout().flush().expect("Could not flush STDOUT");
}

fn encode(bytes: impl Iterator<Item = u8>, clear: bool) {
    for code in bytes.ansi_color_encode() {
        stdout()
            .write_all(code.to_string().as_bytes())
            .expect(STDOUT_WRITE_ERR);
    }

    if clear {
        println!("\x1b[0m ");
    }
}

fn stream_stdin(running: Arc<AtomicBool>, skip_bom: bool) -> impl Iterator<Item = u8> {
    stdin()
        .bytes()
        .take_while(move |byte| byte.is_ok() && running.load(Ordering::SeqCst))
        .map(|byte| byte.unwrap())
        .skip_while(move |byte| skip_bom && UTF_16_BOM.contains(byte))
}
