use std::collections::HashMap;
use std::path::Path;
use super::*;


pub struct SpriteSetBuilder<'a> {
    image_path: &'a Path,
    sprite_bounds: HashMap<String, SpriteBounding>
}

impl<'a> SpriteSetBuilder<'a> {

    pub fn from_file(path: &Path) -> SpriteSetBuilder {
        SpriteSetBuilder {
            image_path: path,
            sprite_bounds: HashMap::new()
        }
    }

    pub fn set_sprite<S>(mut self, id: S, x: u32, y: u32, width: u32, height: u32) -> Self where
        S: Into<String> {

        self.sprite_bounds.insert(id.into(), (x, y, width, height)); self
    }

    pub fn build(self) -> SpriteSet {
        let mut image = image::open(self.image_path).unwrap();

        let sprites = self.sprite_bounds.iter().map(|(id, bounding)| {
            (id.clone(), Sprite::new(image.crop(
                bounding.0,
                bounding.1,
                bounding.2,
                bounding.3
            )))
        }).collect();

        SpriteSet { sprites: sprites }
    }
}
