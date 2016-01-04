// cada widget tiene su propio thread en su thread... despues los veo todos y los mato en la
// LinkedList
extern crate unix_socket;
extern crate log;

use std::io::prelude::*;
use std::thread;
use std::collections::HashSet;
use unix_socket::{UnixStream, UnixListener};
use std::sync::mpsc::{channel, Sender, SendError, Receiver};
use std::clone::Clone;

#[derive(Clone)]
struct IdentifiedSender<T: Send> {
    id: u32,
    tx: Sender<IdentifiedMessage<T>>,
}

impl<T: Send> IdentifiedSender<T> {
    pub fn send(&self, t: T) -> Result<(), SendError<IdentifiedMessage<T>>> {
        self.tx.send(IdentifiedMessage {
            id: self.id,
            data: t,
        })
    }

    pub fn new(id: u32, tx: Sender<IdentifiedMessage<T>>) -> IdentifiedSender<T>{
        IdentifiedSender {
            id: id,
            tx: tx,
        }
    }
}

struct IdentifiedMessage<T: Send> {
    id: u32,
    data: T,
}

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn handle_client(mut stream: UnixStream, tx: IdentifiedSender<Action>) {
    println!("new connection");
    let mut input = String::new();
    let mut buf: [u8;4] = [0,0,0,0];
    let mut i = 0;
    for byte in (&stream).bytes() {
        buf[i] = match byte {
            Ok(b) => b,
            Err(_) => panic!("Failed to read")
        };
        let mut c = 0;
        unsafe {
            c = mem::transmute::<[u8; 4], u32>(buf);
        }
        match char::from_u32(c) {
            Some('\n') => {
                if ! input.starts_with("\\") {
                    tx.send(Action::Update(Field::Content(input.clone())));
                } else {
                    let mut cmd: Vec<&str> = input.split_whitespace().collect();
                    if cmd.get(0).unwrap() == &"\\prio" {
                        match cmd.get(1).unwrap().parse::<u32>() {
                            Ok(i) => {tx.send(Action::Update(Field::Priority(i)));},
                            Err(_) => {},
                        }
                    }
                }
                input = String::new();
            },
            Some(new_char) => { input.push(new_char);},
            None => { i += 1;},
        };
    }
    tx.send(Action::Remove).unwrap();
    println!("connection closed");
}

use std::mem;
use std::char;

enum Field {
    Content(String),
    Priority(u32),
}

enum Action {
    Remove,
    Update(Field),
}

struct Widget {
    priority: u32,
    id: u32,
    content: String,
}

impl Widget {
    pub fn new(id: u32) -> Widget {
        Widget {
            id: id,
            content: String::new(),
            priority: 1024,
        }
    }

    pub fn update(&mut self, field: &Field) {
        match field {
            &Field::Content(ref s) => {self.content = s.clone()},
            &Field::Priority(p) => {self.priority = p},
        }
    }
}

const SEPARATOR: &'static str = " | ";
fn main() {
    let (tx, rx): (Sender<IdentifiedMessage<Action>>, Receiver<IdentifiedMessage<Action>>) = channel();

    thread::spawn(move || {
        let mut vec: Vec<Widget> = Vec::new();

        for msg in rx {
            match msg.data {
                Action::Remove => {
                    vec.retain(|w| w.id != msg.id);
                },
                Action::Update(ref field) => {
                    let mut idPresent = false;
                    for mut widget in vec.iter_mut() {
                        if widget.id == msg.id {
                            widget.update(field);
                            idPresent = true;
                            break
                        }
                    }

                    if ! idPresent {
                        let mut new_widget = Widget::new(msg.id);
                        new_widget.update(field);
                        vec.push(new_widget)
                    }
                }
            }

            vec.sort_by(|a, b| a.priority.cmp(&b.priority));

            println!("update: {}", vec.iter().fold(String::new(), |mut acc, w| {
                if ! w.content.is_empty() {
                    if ! acc.is_empty() {
                        acc.push_str(SEPARATOR)
                    }
                    acc.push_str(&(w.content));
                }
                acc
            }))
        }
    });

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
    // println!("map: {:?}", master_map);
    std::fs::remove_file("socket");
}
