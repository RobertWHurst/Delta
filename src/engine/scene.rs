use super::super::entities::Element;


/// Scenes contain states of your game. Scenes could contain
/// game levels, menus, etc.
pub struct Scene<'a> {
    elements: Vec<Element<'a>>
}

impl<'a> Scene<'a> {

    /// Creates a new scene with a given id
    pub fn new() -> Scene<'a> {
        Scene {
            elements: Vec::new()
        }
    }

    /// Adds an element to the scene
    pub fn add_element(&mut self, element: Element<'a>) {
        self.elements.push(element);
    }

    pub(crate) fn tick(&mut self, tick: f64) {
        for _ in 0..self.elements.len() {
            let mut element = self.elements.swap_remove(0);
            element.tick(self, tick);
            self.elements.push(element);
        }
    }
}
