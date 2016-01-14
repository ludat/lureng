extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str =  "
Usage: lureng [options] -c <cmd>

Options:
    --separator <separator>  Separator between output arguments.
    --socket <socket>        Socket file path to listen to [Default: .lureng.sock].
    -f                       Remove socket file if it exists.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_f: bool,
    flag_socket: String,
    flag_separator: Option<String>,
    arg_cmd: String,
}

#[macro_use]
extern crate unix_socket;

mod widget;
mod id_sender;
mod listener;
mod composer;

use listener::listener;
use composer::composer;

use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    let socket = args.flag_socket;
    if args.flag_f {
        std::fs::remove_file(&socket);
    }
    let (tx, rx) = channel();

    thread::spawn(move || listener(tx, socket));
    // thread::spawn(move || composer(rx, "dzen2")).join().unwrap();
    let separator = &args.flag_separator.unwrap_or(" | ".to_string());
    composer(rx, &args.arg_cmd, separator);
    println!("Amout");
}
