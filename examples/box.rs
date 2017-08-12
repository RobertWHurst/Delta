extern crate delta;

use delta::*;

struct BoxController {
    id: &'static str,
}

impl Controller for BoxController {
    fn tick(&mut self, api: Api) {
        println!(
            "id: {:?} t: {:?}, worker_index: {:?}",
            self.id,
            api.t(),
            api.worker_index()
        );
        // for _ in 0..10000000 {}
    }
}

fn main() {
    let box_ctrl_1 = BoxController { id: "1" };
    let box_ctrl_2 = BoxController { id: "2" };

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
