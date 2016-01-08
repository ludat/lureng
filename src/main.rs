#[macro_use]
extern crate log;
extern crate unix_socket;
extern crate env_logger;

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
    env_logger::init().unwrap();
    info!("deleting stupid file");
    std::fs::remove_file("socket");
    let (tx, rx): (Sender<IdentifiedMessage<Action>>, Receiver<IdentifiedMessage<Action>>) = channel();

    thread::spawn(move || listener(tx));
    // thread::spawn(move || composer(rx, "dzen2")).join().unwrap();
    composer(rx, "dzen2");
    info!("Amout");
}
