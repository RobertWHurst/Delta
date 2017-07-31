use image::DynamicImage;


pub struct Sprite {
    pub image: DynamicImage
}

impl Sprite {

    pub fn new(image: DynamicImage) -> Self {
        Sprite { image: image }
    }
}
