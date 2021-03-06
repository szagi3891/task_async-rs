use std::sync::mpsc::Receiver;

use thread_pool::types::{CounterType};

pub struct ReceiverId<Param> {
    id: CounterType,
    receiver: Receiver<Param>
}

impl<Param> ReceiverId<Param> where Param: Send + Sync + 'static {
    pub fn new(id: CounterType, receiver: Receiver<Param>) -> ReceiverId<Param> {
        ReceiverId {
            id: id,
            receiver: receiver
        }
    }
    
    pub fn id(&self) -> CounterType {
        self.id.clone()
    }

    pub fn recv(&self) -> Option<Param> {
                                                    //TODO - recv_timeout - https://github.com/rust-lang/rfcs/issues/962
        match self.receiver.recv() {
            Ok(param) => Some(param),
            Err(_) => None,
        }
    }
}
