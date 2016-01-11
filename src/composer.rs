use id_sender::*;
use widget::{Widget,Action};

use std::io::prelude::Write;
use std::process::{Command,Stdio};
use std::sync::mpsc::{Receiver};

trait FindById {
    fn find_by_id(&self, id: u32) -> Option<usize>;
}

impl FindById for Vec<Widget> {
    fn find_by_id(&self, id: u32) -> Option<usize> {
        let mut index = 0;
        for item in self {
            if item.id == id {
                return Some(index)
            };
            index += 1;
        };
        return None
    }
}

pub fn composer (rx: Receiver<IdentifiedMessage<Action>>, bar: &str, separator: &str) {
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
        let index = match vec.find_by_id(msg.id) {
            Some(i) => i,
            None => {
                let new_widget = Widget::new(msg.id);
                vec.push(new_widget);
                vec.len() - 1
            }
        };
        match msg.data {
            Action::Remove => {
                vec.remove(index);
            },
            Action::Up => {
                if index > 0 {
                    vec.swap(index, index - 1)
                }
            },
            Action::Down => {
                if index < vec.len() - 1  {
                    vec.swap(index, index + 1)
                }
            },
            Action::Update(ref field) => {
                vec[index].update(field);
            }
        }

        acc.clear();
        for w in vec.iter() {
            if ! w.content.is_empty() {
                if ! acc.is_empty() {
                    acc.push_str(separator)
                }
                acc.push_str(&w.content);
            }
        }
        acc.push('\n');
        out.write(acc.as_bytes()).unwrap();
        out.flush().unwrap();
        println!("Current vec: {:?}", vec);
    }
}
