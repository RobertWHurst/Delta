use std::fmt;
use std::path::PathBuf;
use image::{self, DynamicImage};
use bounds::Bounds;

#[derive(Clone)]
pub struct Sprite {
    source_path: PathBuf,
    source_bounds: Option<Bounds>,
    image: DynamicImage,
}

impl Sprite {
    pub fn from_file<P>(into_path: P) -> Result<Self, image::ImageError>
    where
        P: Into<PathBuf>,
    {
        let path = into_path.into();
        let image = match image::open(&path) {
            Err(e) => return Err(e),
            Ok(d) => d,
        };
        Ok(Self {
            source_path: path,
            source_bounds: None,
            image: image,
        })
    }
    pub fn sub_sprite(&mut self, x: u32, y: u32, width: u32, height: u32) -> Self {
        let source_bounds = match self.source_bounds {
            Some(b) => Bounds::new(b.x + x, b.y + y, width, height),
            None => Bounds::new(x, y, width, height),
        };

        Self {
            source_path: self.source_path.clone(),
            source_bounds: Some(source_bounds),
            image: self.image.crop(x, y, width, height),
        }
    }
}

impl fmt::Debug for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.source_bounds {
            Some(b) => {
                write!(
                    f,
                    "Image {{ source_path: {:?}, source_bounds: {:?} }}",
                    self.source_path,
                    b
                )
            }
            None => write!(f, "Image {{ source_path: {:?} }}", self.source_path),
        }
    }
}

impl PartialEq for Sprite {
    fn eq(&self, other: &Self) -> bool {
        self.source_path == other.source_path && self.source_bounds == other.source_bounds
    }
}

impl Eq for Sprite {}
