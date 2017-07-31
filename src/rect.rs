

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub z: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: f64, y: f64, z: u32, width: u32, height: u32) -> Self {
        Rect {
            x: x,
            y: y,
            z: z,
            width: width,
            height: height,
        }
    }
}
