use std::thread;
use std::mem;
use std::char;
use unix_socket::{UnixStream, UnixListener};
use std::sync::mpsc::Sender;
use std::io::Read;

use id_sender::*;
use widget::{Action,Field};

fn parse_input(input: &String) -> Result<Action, &'static str> {
    if ! input.starts_with("\\") {
        Ok(Action::Update(Field::Content(input.clone())))
    } else {
        let cmd: Vec<&str> = input.split_whitespace().collect();
        match cmd.get(0) {
            Some(&"\\up") => {
                Ok(Action::Up)
            },
            Some(&"\\down") => {
                Ok(Action::Down)
            },
            Some(&_) => Err("Command not found"),
            None => Err("Failed to parse command"),
        }
    }
}

fn handle_client(stream: UnixStream, tx: IdentifiedSender<Action>) {
    println!("new connection");
    let mut input = String::new();
    let mut buf: [u8;4] = [0,0,0,0];
    let mut i = 0;
    for byte in (&stream).bytes() {
        buf[i] = byte.unwrap_or_else(|e| {
            tx.send(Action::Remove);
            panic!("Couldn't read byte ({})", e)
        });
        let c = unsafe {
            mem::transmute::<[u8; 4], u32>(buf)
        };
        match char::from_u32(c) {
            Some('\n') => {
                match parse_input(&input) {
                    Ok(action) => {tx.send(action).unwrap();},
                    Err(ref s) => {println!("Bad command {}", s);},
                };
                input.clear();
            },
            Some(new_char) => { input.push(new_char);},
            None => { i += 1;},
        };
    }
    tx.send(Action::Remove).unwrap();
    println!("connection closed");
}

pub fn listener(tx: Sender<IdentifiedMessage<Action>>) {
    let listener = UnixListener::bind("socket").unwrap();

    // accept connections and process them, spawning a new thread for each one
    let mut id = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                id = id + 1;
                let tx = IdentifiedSender::new(id, tx.clone());
                thread::spawn(move || handle_client(stream, tx));
            }
            Err(_) => {
                /* connection failed */
                break;
            }
        }
    }
    drop(listener);
}
