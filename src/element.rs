use super::*;


#[derive(Debug)]
pub struct Element {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub x: f64,
    pub y: f64,
    pub z: u32,
    pub on_tick: fn(Element, f64),
    pub on_render: fn(Element, f64),
}

impl Element {
    pub fn render(&self) -> RenderEntity {
        RenderEntity::Sprite(self.id.clone())
    }
}
