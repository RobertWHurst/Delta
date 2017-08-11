use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use element::{Element, ElementArc};

pub struct Scene {
    id: String,
    elements: HashMap<String, ElementArc>,
}

impl Scene {
    pub fn new<S>(id: S) -> Scene
    where
        S: Into<String>,
    {
        Scene {
            id: id.into(),
            elements: HashMap::new(),
        }
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.insert(
            element.id.clone(),
            Arc::new(Mutex::new(element)),
        );
    }

    pub fn get_element_groups(&self, num_groups: usize) -> Vec<Vec<ElementArc>> {
        let mut elements: HashMap<_, _> =
            self.elements.iter().map(|r| (r.0, r.1.clone())).collect();

        let mut element_ids: Vec<_> = elements.keys().map(|id| id.clone()).collect();
        let num_elements = elements.len();
        let mut num_elements_per_group = num_elements / num_groups;
        if num_elements % num_groups != 0 {
            num_elements_per_group += 1
        }

        (0..num_groups)
            .into_iter()
            .map(|_| {
                element_ids
                    .drain(..num_elements_per_group)
                    .map(|id| elements.remove(id).unwrap())
                    .collect()
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_element_groups() {}
}
