use std::path::Path;
use std::fmt;

use image::GenericImage;
use image;

use super::sprite::Sprite;


/// Contains a spritesheet image from which sprites can be created.
///
/// Spritesheet should be used to load image data needed for your application where images are
/// combined. If you have a single sprite to load from a individual file then use sprite directly.
pub struct Spritesheet<'a> {
    path: &'a Path,
    image: image::DynamicImage,
}

impl<'a> Spritesheet<'a> {

    /// Creates a new `Spritesheet<'a>` from an image file at the given path
    ///
    /// #Errors
    ///
    /// An error will result if the path does not point to a file that can be
    /// loaded by the library.
    ///
    /// #Examples
    ///
    /// Loading a Character Spritesheet
    ///
    /// ```
    /// let path        = Path::new("assets/sprites/characters/player-1.png");
    /// let spritesheet = Spritesheet::from_file(&path);
    /// ```
    pub fn from_file(path: &Path) -> Result<Spritesheet, image::ImageError> {
        let image = image::open(path)?;

        let spritesheet = Spritesheet {
            path: path,
            image: image,
        };

        Ok(spritesheet)
    }

    /// Creates a new `Sprite` from a sub section of the spritesheet image
    ///
    /// The x, y, w (width), h (height) coordinates are used to crop an area of the
    /// spritesheet. That area is then turned into the source image for the sprite.
    /// If the coordinates are in bounds then the new sprite will result. Otherwise an error.
    pub fn get_sprite_at(&mut self, x: u32, y: u32, w: u32, h: u32) -> Result<Sprite, String> {
        self.validate_bounds(x, y, w, h)?;

        let sprite = Sprite {
            path: self.path,
            image: self.image.crop(x, y, w, h)
        };

        Ok(sprite)
    }

    fn validate_bounds(&self, x: u32, y: u32, w: u32, h: u32) -> Result<(), String> {
        let tl = (x,     y);
        let tr = (x + w, y);
        let bl = (x,     y + h);
        let br = (x + w, y + h);

        if
            self.image.in_bounds(tl.0, tl.1) &&
            self.image.in_bounds(tr.0, tr.1) &&
            self.image.in_bounds(bl.0, bl.1) &&
            self.image.in_bounds(br.0, br.1)
        {
            Ok(())
        } else {
            Err("Out of bounds".to_string())
        }

    }
}

impl<'a> fmt::Debug for Spritesheet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Spritesheet {{ path: {:?} }}", self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_instance<'a>() -> Spritesheet<'a> {
        let path  = Path::new("test_assets/16bitIsometricAdventure/Assets/Characters/character-1.png");
        Spritesheet::from_file(path).expect("failed to load sprite sheet")
    }

    #[test]
    fn from_file() {
        create_instance();
    }

    #[test]
    fn print_debug() {
        assert_eq!(
            format!("{:?}", create_instance()),
            "Spritesheet { path: \"test_assets/16bitIsometricAdventure/Assets/Characters/character-1.png\" }"
        );
    }

    #[test]
    fn get_sprite_at() {
        create_instance().get_sprite_at(0, 0, 16, 24).unwrap();
    }

    #[test]
    fn get_sprite_at_err_out_of_bounds() {
        create_instance().get_sprite_at(0, 0, 160, 240).unwrap_err();
    }
}
