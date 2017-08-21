use std::thread::{spawn, JoinHandle};
use std::sync::{Arc, Mutex, RwLock};
use scene::Scene;
use controller::ControllerApi;

pub struct Worker {
    end_worker_thread: Arc<Mutex<bool>>,
    join_handle: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(worker_index: usize, scene_mx: Arc<RwLock<Option<Scene>>>) -> Self {
        let end_worker_thread = Arc::new(Mutex::new(false));

        Self {
            end_worker_thread: end_worker_thread.clone(),
            join_handle: Some(spawn(move || {
                WorkerController::new(worker_index, end_worker_thread, scene_mx).start()
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
    scene_mx: Arc<RwLock<Option<Scene>>>,
}

impl WorkerController {
    fn new(
        worker_index: usize,
        end_worker_thread: Arc<Mutex<bool>>,
        scene_mx: Arc<RwLock<Option<Scene>>>,
    ) -> Self {
        Self {
            worker_index,
            end_worker_thread,
            scene_mx,
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

            let scene_grd = self.scene_mx.read().unwrap();
            let scene = match *scene_grd {
                Some(ref s) => s,
                None => continue,
            };
            let elements_mx = scene.elements();
            let elements = elements_mx.read().unwrap();

            for element_mx in elements.values() {
                let mut element = match element_mx.try_lock() {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                let id = element.id();
                element.tick(ControllerApi::new(
                    self.worker_index,
                    id,
                    elements_mx.clone(),
                ));
            }
        }
    }
}
