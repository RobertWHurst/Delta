use std::thread::{spawn, JoinHandle};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use element::ElementArc;

type ReceiverArc = Arc<Mutex<Receiver<Message>>>;

enum Message {
    Kill,
    Tick { delta: f64, element_mut: ElementArc },
}

pub struct Worker {
    sender: Sender<Message>,
    join_handle: JoinHandle<()>,
}

impl Worker {
    pub(crate) fn new() -> Worker {
        let (sender, receiver) = channel();
        let receiver_mut = Arc::new(Mutex::new(receiver));
        let receiver_mut_clone = receiver_mut.clone();

        let join_handle = spawn(move || Self::init(receiver_mut_clone));

        Worker {
            sender,
            join_handle,
        }
    }

    pub(crate) fn tick(delta: f64, element_group: ElementArc) {}

    pub(crate) fn kill(&self) -> Result<(), ()> {
        match self.sender.send(Message::Kill) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    fn init(receiver_mut: ReceiverArc) {
        loop {
            let receiver = match receiver_mut.lock() {
                Ok(r) => r,
                Err(_) => break,
            };

            let message = match receiver.recv() {
                Ok(m) => m,
                Err(_) => break,
            };

            let (delta, element_mut) = match message {
                Message::Tick { delta, element_mut } => (delta, element_mut),
                Message::Kill => break,
            };

            let mut element = match element_mut.lock() {
                Ok(e) => e,
                Err(_) => continue,
            };

            print!("Delta: {:?} Element: {:?}", delta, element);
        }
    }
}
