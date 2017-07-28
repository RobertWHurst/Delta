use glutin::EventsLoop as GlutinEventsLoop;
use glutin::Window as GlutinWindow;
use glutin::WindowBuilder as GlutinWindowBuilder;


pub struct Window {
    glutin_events_loop: GlutinEventsLoop,
    glutin_window     : GlutinWindow,
}

impl Window {
    pub fn new() -> Window {
        let events_loop = GlutinEventsLoop::new();

        let window = GlutinWindowBuilder::new()
            .with_title("Ascend")
            .with_dimensions(640, 480)
            .build(&events_loop)
            .expect("Failed to create window");

        Window {
            glutin_events_loop: events_loop,
            glutin_window     : window,
        }
    }
}
