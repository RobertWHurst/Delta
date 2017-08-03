use std::fmt;

use super::super::sprites::Sprite;
use super::super::sprites::Animation;
use super::super::engine::Scene;


/// Element represents any visible element within the engine
pub struct Element<'a> {
    /// The position of the element on the x axis
    pub x: f64,
    /// The position of the element on the y axis
    pub y: f64,
    /// The layer of rendering for the element
    pub layer: u32,
    pub(crate) sprite: Option<&'a Sprite<'a>>,
    pub(crate) animation: Option<&'a mut Animation<'a>>,
    on_tick: Vec<fn(&mut Element, &mut Scene, f64)>
}

impl<'a> Element<'a> {

    /// Creates a new element
    pub fn new() -> Element<'a> {
        Element {
            x: 0.,
            y: 0.,
            layer: 0,
            sprite: None,
            animation: None,
            on_tick: Vec::new(),
        }
    }

    /// Adds an on tick handler
    pub fn on_tick(&mut self, on_tick: fn(&mut Element, &mut Scene, f64)) {
        self.on_tick.push(on_tick);
    }

    /// Sets the element's current sprite
    ///
    /// If an animation was before it clears it
    pub fn set_sprite(&mut self, sprite: &'a Sprite) {
        self.sprite    = Some(sprite);
        self.animation = None;
    }

    /// Sets the element's current animation
    ///
    /// If a sprite was before it clears it
    pub fn set_animation(&mut self, animation: &'a mut Animation<'a>) {
        self.animation = Some(animation);
        self.sprite    = None;
    }

    /// Sets the element's current animation
    ///
    /// If a sprite was before it clears it
    pub fn animation(&mut self) -> Option<&'a mut Animation<'a>> {
        self.animation.take()
    }

    pub(crate) fn tick(&mut self, scene: &mut Scene, tick: f64) {
        for _ in 0..self.on_tick.len() {
            let handler = self.on_tick.swap_remove(0);
            handler(self, scene, tick);
            self.on_tick.push(handler);
        }
    }
}

impl<'a> fmt::Debug for Element<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Element {{ x: {:?}, y: {:?}, layer: {:?}, sprite: {:?}, animation: {:?} }}",
            self.x,
            self.y,
            self.layer,
            self.sprite,
            self.animation
        )
    }
}
