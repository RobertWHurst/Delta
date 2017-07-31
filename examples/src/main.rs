extern crate delta;

use delta::Runtime        as DeltaRuntime;
use delta::Scene          as DeltaScene;
use delta::ElementBuilder as DeltaElementBuilder;

fn main() {

    // create the element
    let box_element = DeltaElementBuilder::new("box")
        .z(10)
        .on_tick(|mut e, d| { e.x += 1. * d; })
        .on_render(|mut e, d| { e.y += 1. * d; })
        .build();

    // create the scene
    let mut s = DeltaScene::new("example");
    s.add_element(box_element);

    // create the runtime
    let mut r = DeltaRuntime::new();
    r.add_scene(s);
    r.set_scene("example").expect("Failed to set scene");

    // start a background thread for the game loop
    r.start();
}
