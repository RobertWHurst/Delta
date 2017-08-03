use std::mem;
use std::fmt;

use super::{Controller,ElementApi};

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
    controller: Box<Controller>,
}

impl<'a> Element<'a> {

    /// Creates a new element
    pub fn new<C>(controller: C) -> Element<'a> where C: 'static + Controller {
        Element {
            x: 0.,
            y: 0.,
            layer: 0,
            sprite: None,
            animation: None,
            controller: Box::new(controller)
        }
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
        let mut api = ElementApi {
            scene: scene
        };
        self.controller.on_tick(&mut api, tick);
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
