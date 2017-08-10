
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Bounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Bounds {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Bounds {
        Bounds {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}
