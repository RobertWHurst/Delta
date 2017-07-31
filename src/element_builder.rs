use super::*;


pub struct ElementBuilder {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub x: f64,
    pub y: f64,
    pub z: u32,
    pub on_tick: fn(Element, f64),
    pub on_render: fn(Element, f64),
}

impl ElementBuilder {

    pub fn new<S>(id: S) -> Self where
        S: Into<String> {
        ElementBuilder {
            id: id.into(),
            width: 0,
            height: 0,
            x: 0.,
            y: 0.,
            z: 0,
            on_tick: |_,_|{},
            on_render: |_,_|{},
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width; self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height; self
    }

    pub fn x(mut self, x: f64) -> Self {
        self.x = x; self
    }

    pub fn y(mut self, y: f64) -> Self {
        self.y = y; self
    }

    pub fn z(mut self, z: u32) -> Self {
        self.z = z; self
    }

    pub fn on_tick(mut self, on_tick: fn(Element, f64)) -> Self {
        self.on_tick = on_tick; self
    }

    pub fn on_render(mut self, on_render: fn(Element, f64)) -> Self {
        self.on_render = on_render; self
    }

    pub fn build(self) -> Element {
        Element {
            id: self.id,
            width: self.width,
            height: self.height,
            x: self.x,
            y: self.y,
            z: self.z,
            on_tick: self.on_tick,
            on_render: self.on_render,
        }
    }
}
