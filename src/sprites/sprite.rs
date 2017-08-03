use std::path::Path;
use std::fmt;

use image::{DynamicImage,GenericImage};
use image;


/// Contains a single sprite
///
/// A sprite can be attached to an element or it can be added to an animation.
pub struct Sprite<'a> {
    pub(super) path: &'a Path,
    pub(super) image: DynamicImage
}

impl<'a> Sprite<'a> {

    /// Creates a single sprite from a file
    pub fn from_file(path: &Path) -> Result<Sprite, image::ImageError> {
        let image = image::open(path)?;

        let sprite = Sprite {
            path: path,
            image: image,
        };

        Ok(sprite)
    }
}

impl<'a> fmt::Debug for Sprite<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sprite {{ path: {:?} }}", self.path)
    }
}

impl<'a> GenericImage for Sprite<'a> {
    type Pixel = image::Rgba<u8>;

    fn dimensions(&self) -> (u32, u32) {
        self.image.dimensions()
    }

    fn bounds(&self) -> (u32, u32, u32, u32) {
        self.image.bounds()
    }

    fn get_pixel(&self, x: u32, y: u32) -> image::Rgba<u8> {
        self.image.get_pixel(x, y)
    }

    fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut image::Rgba<u8> {
        self.image.get_pixel_mut(x, y)
    }

    fn put_pixel(&mut self, x: u32, y: u32, pixel: image::Rgba<u8>) {
        self.image.put_pixel(x, y, pixel)
    }

    fn blend_pixel(&mut self, x: u32, y: u32, pixel: image::Rgba<u8>) {
        self.image.blend_pixel(x, y, pixel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_debug() {
        let sprite = Sprite {
            path: &Path("__PATH__"),
            image: DynamicImage::new_rgba(100, 100)
        };
        assert_eq!(
            format!("{:?}", sprite),
            "Sprite { path: \"__PATH__\" }"
        );
    }
}
