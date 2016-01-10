use id_sender::*;
use widget::{Widget,Action};

use std::io::prelude::Write;
use std::process::{Command,Stdio};
use std::sync::mpsc::{Receiver};

const SEPARATOR: &'static str = " | ";

pub fn composer (rx: Receiver<IdentifiedMessage<Action>>, bar: &str) {
    let mut vec: Vec<Widget> = Vec::with_capacity(16);
    let child = Command::new("sh")
        .arg("-c")
        .arg(bar)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut out = child.stdin.unwrap();
    let mut acc = String::new();

    for msg in rx {
        match msg.data {
            Action::Remove => {
                vec.retain(|w| w.id != msg.id);
            },
            Action::Update(ref field) => {
                let mut id_present = false;
                for mut widget in vec.iter_mut() {
                    if widget.id == msg.id {
                        widget.update(field);
                        id_present = true;
                        break
                    }
                }

                if ! id_present {
                    let mut new_widget = Widget::new(msg.id);
                    new_widget.update(field);
                    vec.push(new_widget)
                }
            }
        }

        // sort vector by priority because I'm really lazy (optimization is the root of all evil)
        vec.sort_by(|a, b| a.priority.cmp(&b.priority));

        acc.clear();
        for w in vec.iter() {
            if ! w.content.is_empty() {
                if ! acc.is_empty() {
                    acc.push_str(SEPARATOR)
                }
                acc.push_str(&w.content);
            }
        }
        acc.push('\n');
        out.write(acc.as_bytes()).unwrap();
        out.flush().unwrap();
    }
}
