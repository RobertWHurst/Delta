use std::collections::HashMap;
use super::*;


pub struct SpriteSet {
    pub sprites: HashMap<String, Sprite>
}

impl SpriteSet {

    pub fn new(sprites: HashMap<String, Sprite>) -> Self {
        SpriteSet { sprites: sprites }
    }
}
