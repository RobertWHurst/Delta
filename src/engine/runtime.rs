use time::PreciseTime;
use super::Scene;


/// The heart of Delta, the Runtime is responsible for managing
/// the current scene, and window. It also manages worker threads
pub struct Runtime<'a> {
    scene: Option<Scene<'a>>,
    last_tick_at: Option<PreciseTime>
}

impl<'a> Runtime<'a> {

    /// Creates a new runtime
    pub fn new() -> Runtime<'a> {
        Runtime {
            scene: None,
            last_tick_at: None
        }
    }

    /// Sets a new scene. If a previous scene was present it will return it
    pub fn set_scene(&mut self, scene: Scene<'a>) -> Option<Scene<'a>> {
        let previous_scene = self.scene.take();
        self.scene = Some(scene);
        previous_scene
    }

    /// Executes just one tick
    pub fn tick(&mut self, tick: f64) {
        self.scene.as_mut().expect("Scene not set").tick(tick);
    }

    /// Starts the runtime
    pub fn start(&mut self) {

        loop {
            if let None = self.scene {
                continue;
            }

            let tick_at = PreciseTime::now();

            // NOTE: On the first tick we simply set the last tick time
            if let None = self.last_tick_at {
                self.last_tick_at = Some(tick_at);
                continue;
            }

            let last_tick_at  = self.last_tick_at.unwrap();
            let tick_duration = last_tick_at.to(tick_at);

            let delta_time_nanos   = tick_duration.num_nanoseconds().unwrap_or(i64::max_value());
            let delta_time_seconds = delta_time_nanos as f64 / 1000. / 1000. / 1000.;

            self.last_tick_at = Some(tick_at);

            self.tick(delta_time_seconds);
        }
    }
}
