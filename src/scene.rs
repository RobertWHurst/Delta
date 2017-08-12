use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use element::Element;

pub struct Scene {
    id: String,
    elements: Arc<RwLock<HashMap<String, Arc<Mutex<Element>>>>>,
}

impl Scene {
    pub fn new<S>(id: S) -> Scene
    where
        S: Into<String>,
    {
        Scene {
            id: id.into(),
            elements: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_elements(&self) -> Arc<RwLock<HashMap<String, Arc<Mutex<Element>>>>> {
        self.elements.clone()
    }

    pub fn add_element(&self, element: Element) -> Result<(), ()> {
        let mut elements = self.elements.write().unwrap();
        if elements.contains_key(&element.id) {
            return Err(());
        }
        elements.insert(element.id.clone(), Arc::new(Mutex::new(element)));
        Ok(())
    }

    pub fn get_element<S>(&self, id: S) -> Option<Arc<Mutex<Element>>>
    where
        S: Into<String>,
    {
        let elements = self.elements.read().unwrap();
        match elements.get(&id.into()) {
            Some(e) => Some(e.to_owned()),
            None => None,
        }
    }

    pub fn remove_element<S>(&self, id: S) -> Option<()>
    where
        S: Into<String>,
    {
        match self.elements.write().unwrap().remove(&id.into()) {
            Some(_) => Some(()),
            None => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_element_groups() {}
}
