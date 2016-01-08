use std::sync::mpsc::{Sender, SendError};

#[derive(Clone)]
pub struct IdentifiedSender<T: Send> {
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

pub struct IdentifiedMessage<T: Send> {
    pub id: u32,
    pub data: T,
}
