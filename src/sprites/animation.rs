use super::sprite::Sprite;


/// An animation is an ordered set of sprites that can be used with an element
#[derive(Debug)]
pub struct Animation<'a> {
    sprites: Vec<&'a Sprite<'a>>,
    current_sprite: usize
}

impl<'a> Animation<'a> {

    /// Create a new animation
    pub fn new() -> Animation<'a> {
        Animation {
            sprites: Vec::new(),
            current_sprite: 0
        }
    }

    /// Add a sprite to the end of the animation
    pub fn add_sprite(&mut self, sprite: &'a Sprite) {
        self.sprites.push(sprite);
    }

    /// Returns the current sprite and advances the internal cursor
    pub fn next(&mut self) -> &'a Sprite {
        let sprite = self.sprites[self.current_sprite];
        if self.current_sprite < self.sprites.len() - 1 {
            self.current_sprite += 1;
        } else {
            self.current_sprite = 0;
        }
        sprite
    }

    /// Set the internal cursor
    pub fn set(&mut self, index: usize) -> Result<(), String> {
        if index > self.sprites.len() - 1 {
            return Err(format!("{:?} is out of bounds", index));
        }
        self.current_sprite = index;
        Ok(())
    }

    /// Returns the length of the animation
    pub fn len(&self) -> usize {
        self.sprites.len()
    }
}
