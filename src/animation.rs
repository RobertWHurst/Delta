use std::collections::HashMap;
use std::time::Duration;
use super::*;


pub struct Animation<'a> {
    pub sprites: HashMap<String, &'a Sprite>,
    pub frames: Vec<String>,
    pub frame_duration: Duration,
}
