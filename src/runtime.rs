use std::collections::HashMap;
use error::DeltaError;
use scene::{Scene,SceneRenderEntity};
use window::Window;


pub struct Runtime {
    window            : Option<Window>,
    current_scene_name: Option<&'static str>,
    next_scene_name   : Option<&'static str>,
    scenes            : HashMap<&'static str, Scene>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            window            : None,
            current_scene_name: None,
            next_scene_name   : None,
            scenes            : HashMap::new(),
        }
    }
    pub fn open_window(&mut self) {
        let window = Window::new();
        self.window = Some(window);
    }
    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.insert(scene.id, scene);
    }
    pub fn set_scene(&mut self, scene_name: &'static str) -> Result<(), DeltaError> {
        if !self.scenes.contains_key(scene_name) {
            return Result::Err(DeltaError::UnknownScene);
        }
        if let None = self.current_scene_name {
            self.current_scene_name = Some(scene_name);
        } else {
            self.next_scene_name = Some(scene_name);
        }
        Result::Ok(())
    }
    pub fn start(&mut self) {
        let current_scene_name = self.current_scene_name.expect("Cannot start without a scene set");
        let current_scene      = self.scenes.get_mut(current_scene_name).unwrap();

        let render = current_scene.render(640, 480);

        let ids: Vec<Vec<&'static str>> = render.iter().map(|l| {
            l.iter().map(|s| {
                match s {
                    &SceneRenderEntity::Sprite(id) => id,
                    &SceneRenderEntity::Polygon => ""
                }
            }).collect()
        }).collect();

        println!("{:?}", ids);
    }
}
