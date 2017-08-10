use std::cmp;
use std::fmt;
use position::Position;
use sprite::Sprite;
use controller::Controller;
use velocity::Velocity;

pub struct Element {
    id: String,
    pub(crate) position: Position,
    pub(crate) velocity: Velocity,
    sprite: Option<Sprite>,
    controller: Box<Controller>,
}

impl Element {
    pub fn new<S, C>(id: S, controller: C) -> Self
    where
        S: Into<String>,
        C: 'static + Controller,
    {
        Self {
            id: id.into(),
            position: Position::default(),
            velocity: Velocity::default(),
            sprite: None,
            controller: box controller,
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Element {}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Element {{ id: {} position: {:?}, velocity: {:?} }}",
            self.id,
            self.position,
            self.velocity,
        )
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
