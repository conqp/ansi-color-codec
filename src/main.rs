use ansi_color_codec::{Codec, RESET};
use clap::Parser;
use ctrlc::set_handler;
use std::io::{stdin, stdout, Read, Write};
use std::iter::once;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long, name = "decode")]
    decode: bool,

    #[clap(short, long, name = "no-clear")]
    no_clear: bool,

    #[clap(short, long)]
    skip: Vec<char>,
}

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
        encode(bytes, dst, !args.no_clear, &args.skip);
    }
}

fn decode(bytes: impl Iterator<Item = u8>, mut dst: impl Write) {
    bytes
        .decode()
        .enumerate()
        .filter_map(|(index, result)| match result {
            Ok(byte) => Some(byte),
            Err(error) => {
                eprintln!("{error} at {index}");
                None
            }
        })
        .map_while(|byte| dst.write_all(&[byte]).ok())
        .for_each(drop);

    dst.flush().unwrap_or_else(drop); // Ignore write errors here.
}

fn encode(bytes: impl Iterator<Item = u8>, mut dst: impl Write, clear: bool, skip: &[char]) {
    bytes
        .map_while(|byte| {
            if skip.contains(&(byte as char)) {
                if clear {
                    dst.write_all(RESET.as_bytes()).ok()?;
                }

                dst.write_all(&[byte]).ok()
            } else {
                for code in once(byte).encode() {
                    write!(dst, "{code}").ok()?;
                }

                Some(())
            }
        })
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
