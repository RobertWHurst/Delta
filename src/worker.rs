use std::thread::{spawn, JoinHandle};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use element::ElementArc;

type InnerSenderArc = Arc<Mutex<Sender<OutMessage>>>;
type InnerReceiverArc = Arc<Mutex<Receiver<InMessage>>>;

enum InMessage {
    Exit,
    RequestedElement { id: String, element_mut: ElementArc },
    ExecuteElementTick { delta: f64, element_mut: ElementArc },
}

enum OutMessage {
    GetElement { id: String },
    DestroyElement { id: String },
    CreateElement { element_mut: ElementArc },
}

pub struct WorkerApi {
    sender_mut: InnerSenderArc,
    receiver_mut: InnerReceiverArc,
}

impl WorkerApi {
    fn new(inner_sender: Sender<OutMessage>, inner_receiver: Receiver<InMessage>) -> Self {
        let inner_sender_mut = Arc::new(Mutex::new(inner_sender));
        let inner_receiver_mut = Arc::new(Mutex::new(inner_receiver));

        Self {
            sender_mut: inner_sender_mut.clone(),
            receiver_mut: inner_receiver_mut.clone(),
        }
    }

    fn recv(&self) -> Result<InMessage, ()> {
        let reciever = match self.receiver_mut.lock() {
            Ok(r) => r,
            Err(_) => return Err(()),
        };
        match reciever.recv() {
            Ok(m) => Ok(m),
            Err(_) => Err(()),
        }
    }
}

pub struct Worker {
    sender: Sender<InMessage>,
    receiver: Receiver<OutMessage>,
    join_handle: JoinHandle<()>,
}

impl Worker {
    pub(crate) fn new() -> Worker {
        let (inner_sender, outer_receiver) = channel();
        let (outer_sender, inner_receiver) = channel();

        let api = WorkerApi::new(inner_sender, inner_receiver);

        let join_handle = spawn(move || Self::init(api));

        Worker {
            sender: outer_sender,
            receiver: outer_receiver,
            join_handle,
        }
    }

    pub(crate) fn tick(delta: f64, element_group: ElementArc) {}

    pub(crate) fn kill(&self) -> Result<(), ()> {
        match self.sender.send(InMessage::Exit) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    fn init(api: WorkerApi) {
        loop {
            let message = match api.recv() {
                Ok(m) => m,
                Err(_) => break,
            };

            let (delta, element_mut) = match message {
                InMessage::ExecuteElementTick { delta, element_mut } => (delta, element_mut),
                InMessage::Exit => break,
                InMessage::RequestedElement { id, element_mut } => unreachable!(),
            };

            let mut element = match element_mut.lock() {
                Ok(e) => e,
                Err(_) => continue,
            };


            print!("Delta: {:?} Element: {:?}", delta, element);
        }
    }
}
