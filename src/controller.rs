use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use element::{Element, ElementApi, ElementData};

pub trait Controller {
    fn tick(&mut self, ElementApi, ControllerApi) {}
    fn load(&mut self, ElementApi, ControllerApi) {}
    fn unload(&mut self, ElementApi, ControllerApi) {}
    fn message(&mut self, ElementApi, ControllerApi) {}
}

pub struct ControllerApi {
    worker_index: usize,
    element_id: String,
    elements_mx: Arc<RwLock<HashMap<String, Arc<Mutex<Element>>>>>,
    tick_delta: f64,
}

impl ControllerApi {
    pub fn new<S>(
        worker_index: usize,
        element_id: S,
        elements_mx: Arc<RwLock<HashMap<String, Arc<Mutex<Element>>>>>,
    ) -> Self
    where
        S: Into<String>,
    {
        Self {
            worker_index,
            element_id: element_id.into(),
            elements_mx: elements_mx,
            tick_delta: 0.,
        }
    }

    pub fn worker_index(&self) -> usize {
        self.worker_index
    }

    pub fn t(&self) -> f64 {
        self.tick_delta
    }

    pub fn get_element<S>(&self, into_id: S) -> Option<ElementApi>
    where
        S: Into<String>,
    {
        let id = into_id.into();
        if id == self.element_id {
            return None;
        }

        let elements = self.elements_mx.read().unwrap();
        let element_mx = match elements.get(&id) {
            Some(e) => e.to_owned(),
            None => return None,
        };

        Some(ElementApi::new(id, ElementData::Element(element_mx)))
    }

    pub fn set_tick_delta(&mut self, tick_delta: f64) {
        self.tick_delta = tick_delta;
    }
}
