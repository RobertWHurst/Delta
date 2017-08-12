extern crate delta;

use delta::*;

struct BoxController {}

impl Controller for BoxController {
    fn tick(&mut self, element_api: ElementApi, controller_api: ControllerApi) {
        println!(
            "thread: {:?}, element_id: {:?} tick_delta: {:?}",
            controller_api.worker_index(),
            element_api.id(),
            controller_api.t(),
        );
        // if element_api.id() == "box1" {
        //     println!("{:?}", controller_api.get_element("box2"));
        // }
        // for _ in 0..10000000 {}
    }
}

fn main() {
    let box_ctrl_1 = BoxController {};
    let box_ctrl_2 = BoxController {};

    let box_el_1 = Element::new("box1", box_ctrl_1);
    let box_el_2 = Element::new("box2", box_ctrl_2);

    let mut scene = Scene::new("box");
    scene.add_element(box_el_1).unwrap();
    scene.add_element(box_el_2).unwrap();

    let mut runtime = Runtime::new();
    runtime.set_scene(scene);

    runtime.start(None);

    loop {}
}
