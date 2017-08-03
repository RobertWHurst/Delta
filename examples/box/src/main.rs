extern crate delta;

use std::path::Path;
use delta::*;

fn main() {
    let box_sprite = sprites::Sprite::from_file(&Path::new("assets/box.png"))
        .expect("Failed to load sprite for box");

    let mut element_1 = entities::Element::new();
    element_1.set_sprite(&box_sprite);
    element_1.on_tick(|e, _, t| {
        e.x += 10. * t;
        println!("Element_1 {:?}", e);
    });

    let mut element_2 = entities::Element::new();
    element_2.set_sprite(&box_sprite);
    element_2.on_tick(|_, _, t| {
        println!("From the element_2 {:?}", t);
    });

    let mut scene = Scene::new();
    scene.add_element(element_1);
    // scene.add_element(element_2);

    let mut runtime = Runtime::new();
    runtime.set_scene(scene);

    // runtime.set_thread_pool_size();

    // TODO: This should return an exit reason of some sort
    runtime.start();
}
