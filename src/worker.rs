use std::collections::HashMap;
use std::thread::{spawn, JoinHandle};
use std::sync::{Arc, Mutex, RwLock};
use element::Element;
use controller::Api;

pub struct Worker {
    end_worker_thread: Arc<Mutex<bool>>,
    join_handle: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        worker_index: usize,
        elements_mx: Arc<RwLock<HashMap<String, Arc<Mutex<Element>>>>>,
    ) -> Self {
        let end_worker_thread = Arc::new(Mutex::new(false));

        Self {
            end_worker_thread: end_worker_thread.clone(),
            join_handle: Some(spawn(move || {
                WorkerController::new(worker_index, end_worker_thread, elements_mx).start()
            })),
        }
    }

    pub fn end(&mut self) {
        *self.end_worker_thread.lock().unwrap() = true;
        self.join_handle.take().unwrap().join().unwrap();
    }
}

struct WorkerController {
    worker_index: usize,
    end_worker_thread: Arc<Mutex<bool>>,
    elements_mx: Arc<RwLock<HashMap<String, Arc<Mutex<Element>>>>>,
}

impl WorkerController {
    fn new(
        worker_index: usize,
        end_worker_thread: Arc<Mutex<bool>>,
        elements_mx: Arc<RwLock<HashMap<String, Arc<Mutex<Element>>>>>,
    ) -> Self {
        Self {
            worker_index,
            end_worker_thread,
            elements_mx,
        }
    }
    fn should_exit(&self) -> bool {
        *self.end_worker_thread.lock().unwrap()
    }
    fn start(&self) {
        loop {
            if self.should_exit() {
                break;
            }

            let elements = self.elements_mx.read().unwrap();

            for element_mx in elements.values() {
                let api = Api::new(self.worker_index, self.elements_mx.clone());
                let mut element = element_mx.lock().unwrap();
                element.tick(api);
            }
        }
    }
}
