use element::Element;
use position::Position;
use velocity::Velocity;

pub trait Controller {
    fn tick(&mut self, fn(ControllerApi)) {}
    fn load(&mut self, fn(ControllerApi)) {}
    fn unload(&mut self, fn(ControllerApi)) {}
    fn message(&mut self, fn(ControllerApi)) {}
}

pub struct ControllerApi {
    element_position: Position,
    element_velocity: Velocity,
}

impl ControllerApi {
    pub fn new<C>(element: &Element) -> Self {
        Self {
            element_position: element.position,
            element_velocity: element.velocity,
        }
    }
}
