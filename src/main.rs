#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate rustc_serialize;
extern crate docopt;

docopt!(Args, "Usage: lureng -c <cmd>", arg_x: i32, arg_y: i32);

#[macro_use]
extern crate unix_socket;

mod widget;
mod id_sender;
mod listener;
mod composer;

use id_sender::*;
use widget::{Action};
use listener::listener;
use composer::composer;

use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("deleting stupid file");
    std::fs::remove_file("socket");
    let (tx, rx): (Sender<IdentifiedMessage<Action>>, Receiver<IdentifiedMessage<Action>>) = channel();

    thread::spawn(move || listener(tx));
    // thread::spawn(move || composer(rx, "dzen2")).join().unwrap();
    composer(rx, &args.arg_cmd);
    println!("Amout");
}
