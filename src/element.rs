use std::collections::hash_map::{self, HashMap};
use rayon::prelude::*;
use sprite::Sprite;

#[derive(Clone)]
pub struct Element<C> {
    id: String,
    position: Position,
    velocity: Velocity,
    sprite: Option<Sprite>,
    controller: C,
}

impl<C> Element<C>
where
    C: Controller,
{
    pub fn new<S>(id: S, controller: C) -> Element<C>
    where
        S: Into<String>,
    {
        Element {
            id: id.into(),
            position: Position::default(),
            velocity: Velocity::default(),
            sprite: None,
            controller: controller,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub layer: u8,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Velocity {
    pub rad: f64,
    pub amp: f64,
}

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
    pub fn new<C>(element: &Element<C>) -> Self {
        Self {
            element_position: element.position,
            element_velocity: element.velocity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct BoxController {}
    impl Controller for BoxController {}

    #[test]
    fn create_element() {
        let box_controller = BoxController {};

        Element::new("box", box_controller);
    }
}
