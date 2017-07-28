extern crate delta;

use delta::Runtime        as DeltaRuntime;
use delta::Scene          as DeltaScene;
use delta::ElementBuilder as DeltaElementBuilder;

fn main() {

    // create the element
    let e1 = DeltaElementBuilder::new("top-box")
        .z(10.)
        .on_tick(|mut e, d| { e.x += 1. * d; })
        .on_render(|mut e, d| { e.y += 1. * d; })
        .build();
    let e2 = DeltaElementBuilder::new("bottom-box")
        .z(1.)
        .build();

    // create the scene
    let mut s = DeltaScene::new("example");
    s.add_element(e1);
    s.add_element(e2);
    println!("{:?}", s.remove_element("top-box"));

    // create the runtime
    let mut r = DeltaRuntime::new();
    r.add_scene(s);
    r.set_scene("example").expect("Failed to set scene");

    // start a background thread for the game loop
    r.start();
}
