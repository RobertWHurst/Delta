use scene::Scene;
use worker::Worker;
use num_cpus;

pub struct Runtime {
    scene: Option<Scene>,
    tick_workers: Vec<Worker>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            scene: None,
            tick_workers: Vec::new(),
        }
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = Some(scene);
    }

    pub fn start(&mut self, num_worker_threads_opt: Option<usize>) {
        let num_worker_threads = match num_worker_threads_opt {
            Some(n) => n,
            None => num_cpus::get() - 2,
        };

        self.tick_workers = (0..num_worker_threads)
            .into_iter()
            .map(|i| Worker::new(i, self.scene.as_ref().unwrap().elements()))
            .collect();
    }
}
