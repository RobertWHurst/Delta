use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use element::Element;

pub trait Controller {
    fn tick(&mut self, Api) {}
    fn load(&mut self, Api) {}
    fn unload(&mut self, Api) {}
    fn message(&mut self, Api) {}
}

pub struct Api {
    worker_index: usize,
    elements_mx: Arc<RwLock<HashMap<String, Arc<Mutex<Element>>>>>,
    tick_delta: f64,
}

impl Api {
    pub fn new(
        worker_index: usize,
        elements_mx: Arc<RwLock<HashMap<String, Arc<Mutex<Element>>>>>,
    ) -> Self {
        Self {
            worker_index,
            elements_mx,
            tick_delta: 0.,
        }
    }

    pub fn worker_index(&self) -> usize {
        self.worker_index
    }

    pub fn t(&self) -> f64 {
        self.tick_delta
    }

    pub fn get_element<S>(&self, id: S) -> Option<Arc<Mutex<Element>>>
    where
        S: Into<String>,
    {
        let elements_mx = self.elements_mx.read().unwrap();
        match elements_mx.get(&id.into()) {
            Some(e) => Some(e.to_owned()),
            None => None,
        }
    }

    pub fn set_tick_delta(&mut self, tick_delta: f64) {
        self.tick_delta = tick_delta;
    }
}
