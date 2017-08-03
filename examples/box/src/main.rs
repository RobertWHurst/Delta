extern crate delta;

use std::path::Path;
use delta::*;

struct Element1Controller {

}

impl Element1Controller {
    fn new() -> Element1Controller {
        Element1Controller {}
    }
}

impl entities::Controller for Element1Controller {
    fn on_tick(&mut self, tick: f64) {
        let scene = self.scene();

        println!("TICK {:?}", tick);
    }
}

fn main() {
    let box_sprite = sprites::Sprite::from_file(&Path::new("assets/box.png"))
        .expect("Failed to load sprite for box");

    let controller = Element1Controller::new();
    let mut element_1  = entities::Element::new(controller);
    element_1.set_sprite(&box_sprite);

    let mut scene = Scene::new();
    scene.add_element(element_1);
    // scene.add_element(element_2);

    let mut runtime = Runtime::new();
    runtime.set_scene(scene);

    // runtime.set_thread_pool_size();

    // TODO: This should return an exit reason of some sort
    runtime.start();
}
