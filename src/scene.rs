use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use super::*;


pub struct Scene {
    pub id  : String,
    elements: HashMap<String, Arc<Mutex<Element>>>,
}

impl Scene {
    pub fn new<S: Into<String>>(id: S) -> Scene {
        Scene {
            id      : id.into(),
            elements: HashMap::new()
        }
    }
    pub fn render<F: Into<f64>>(&self, width_f: F, height_f: F) -> Vec<Vec<RenderEntity>> {
        let width              = width_f.into();
        let height             = height_f.into();
        let z_range            = self.get_z_range();
        let mut element_layers = Vec::with_capacity(z_range.len());
        for z in z_range {
            let mut element_layer = Vec::new();
            for element_arc in self.elements.values() {
                let element_clone = element_arc.clone();
                let element       = element_clone.lock().unwrap();
                if element.x < width && element.y < height && element.z == z {
                    element_layer.push(element.render());
                }
            }
            element_layers.push(element_layer);
        }
        element_layers
    }
    pub fn add_element(&mut self, element: Element) {
        self.elements.insert(element.id.clone(), Arc::new(Mutex::new(element)));
    }
    pub fn remove_element<'a, S: Into<&'a str>>(&mut self, element_id: S) -> Option<Element> {
        let mut element_arc = match self.elements.remove(element_id.into()) {
            Some(element_arc) => element_arc,
            None              => { return None; },
        };

        let element_mut = loop {
            match Arc::try_unwrap(element_arc) {
                Ok(element_mut) => { break element_mut; },
                Err(_element_arc) => { element_arc = _element_arc }
            }
        };

        match element_mut.into_inner() {
            Ok(element) => Some(element),
            Err(_)      => None
        }
    }
    fn get_z_range(&self) -> Vec<u32> {
        let mut z_values = Vec::new();
        for element_arc in self.elements.values() {
            let element_clone = element_arc.clone();
            let element       = element_clone.lock().unwrap();
            if !z_values.contains(&element.z) {
                z_values.push(element.z);
            }
        }
        z_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        z_values
    }
}
