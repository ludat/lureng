#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate rustc_serialize;
extern crate docopt;

docopt!(Args, "
Usage: lureng [options] -c <cmd>

Options:
    -s <separator>       Separator between output arguments.
", flag_s: Option<String>);

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
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    std::fs::remove_file("socket");
    let (tx, rx) = channel();

    thread::spawn(move || listener(tx));
    // thread::spawn(move || composer(rx, "dzen2")).join().unwrap();
    let separator = &args.flag_s.unwrap_or(" | ".to_string());
    composer(rx, &args.arg_cmd, separator);
    println!("Amout");
}
