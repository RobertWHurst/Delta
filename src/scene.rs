use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use element::Element;


pub enum SceneRenderEntity {
    Sprite(&'static str),
    Polygon,
}

pub struct Scene {
    pub id  : &'static str,
    elements: HashMap<&'static str, Arc<Mutex<Element>>>,
}

impl Scene {
    pub fn new(id: &'static str) -> Scene {
        Scene {
            id      : id,
            elements: HashMap::new()
        }
    }
    pub fn render(&self, width: usize, height: usize) -> Vec<Vec<SceneRenderEntity>> {
        let z_range            = self.get_z_range();
        let mut element_layers = Vec::with_capacity(z_range.len());
        for z in z_range {
            let mut element_layer = Vec::new();
            for element_arc in self.elements.values() {
                let element_clone = element_arc.clone();
                let element       = element_clone.lock().unwrap();
                if element.x < width as f64 && element.y < height as f64 && element.z == z {
                    element_layer.push(element.render());
                }
            }
            element_layers.push(element_layer);
        }
        element_layers
    }
    pub fn add_element(&mut self, element: Element) {
        self.elements.insert(element.id, Arc::new(Mutex::new(element)));
    }
    pub fn remove_element(&mut self, element_id: &'static str) -> Option<Element> {
        let mut element_arc = match self.elements.remove(element_id) {
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
    fn get_z_range(&self) -> Vec<f64> {
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
