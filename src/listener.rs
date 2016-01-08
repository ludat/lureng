use std::thread;
use std::mem;
use std::char;
use unix_socket::{UnixStream, UnixListener};
use std::sync::mpsc::Sender;
use std::io::Read;

use id_sender::*;
use widget::{Action,Field};

fn handle_client(stream: UnixStream, tx: IdentifiedSender<Action>) {
    debug!("new connection");
    let mut input = String::new();
    let mut buf: [u8;4] = [0,0,0,0];
    let mut i = 0;
    for byte in (&stream).bytes() {
        buf[i] = match byte {
            Ok(b) => b,
            Err(_) => panic!("Failed to read")
        };
        let c;
        unsafe {
            c = mem::transmute::<[u8; 4], u32>(buf);
        }
        match char::from_u32(c) {
            Some('\n') => {
                if ! input.starts_with("\\") {
                    tx.send(Action::Update(Field::Content(input.clone()))).unwrap();
                } else {
                    let cmd: Vec<&str> = input.split_whitespace().collect();
                    if cmd.get(0).unwrap() == &"\\prio" {
                        match cmd.get(1).unwrap().parse::<u32>() {
                            Ok(i) => {
                                tx.send(Action::Update(Field::Priority(i))).unwrap();
                            },
                            Err(_) => {},
                        }
                    }
                }
                input.clear();
            },
            Some(new_char) => { input.push(new_char);},
            None => { i += 1;},
        };
    }
    tx.send(Action::Remove).unwrap();
    debug!("connection closed");
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
