use std::collections::HashMap;
use super::*;


pub struct Runtime {
    window            : Option<Window>,
    current_scene_name: Option<String>,
    next_scene_name   : Option<String>,
    scenes            : HashMap<String, Scene>,
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
        self.scenes.insert(scene.id.clone(), scene);
    }

    pub fn set_scene<S>(&mut self, scene_name_s: S) -> Result<(), String> where
        S: Into<String> {
        let scene_name = scene_name_s.into();

        if !self.scenes.contains_key(&scene_name) {
            return Result::Err(format!("{} is not a registered scene", scene_name));
        }

        if let None = self.current_scene_name {
            self.current_scene_name = Some(scene_name);
        } else {
            self.next_scene_name = Some(scene_name);
        }

        Result::Ok(())
    }

    pub fn start(&mut self) {
        let current_scene_name = self.current_scene_name.as_ref().expect("Cannot start without a scene set");
        let current_scene      = self.scenes.get_mut(current_scene_name).unwrap();
        let render             = current_scene.render(640, 480);

        let ids: Vec<Vec<String>> = render.iter().map(|l| {
            l.iter().map(|s| {
                match s {
                    &RenderEntity::Sprite(ref id) => id.clone(),
                    &RenderEntity::Polygon => "".to_string()
                }
            }).collect()
        }).collect();

        println!("{:?}", ids);
    }
}
