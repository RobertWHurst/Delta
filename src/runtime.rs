use std::sync::{Arc, RwLock};
use scene::Scene;
use worker::Worker;
use num_cpus;

pub struct Runtime {
    scene_mx: Arc<RwLock<Option<Scene>>>,
    tick_workers: Vec<Worker>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            scene_mx: Arc::new(RwLock::new(None)),
            tick_workers: Vec::new(),
        }
    }

    pub fn set_scene(&mut self, scene: Scene) {
        let mut scene_opt = self.scene_mx.write().unwrap();

        // if let Some(previous_scene) = scene_opt {
        //     previous_scene
        // }

        *scene_opt = Some(scene);
    }

    pub fn start(&mut self, num_worker_threads_opt: Option<usize>) {
        let num_worker_threads = match num_worker_threads_opt {
            Some(n) => n,
            None => num_cpus::get() - 2,
        };

        self.tick_workers = (0..num_worker_threads)
            .into_iter()
            .map(|i| Worker::new(i, self.scene_mx.clone()))
            .collect();
    }
}
