use super::ElementApi;
use super::Element;
use super::super::Scene;

pub trait Controller {
    // TODO: Controller API
    fn on_load<'a>(&mut self, &'a mut ElementApi, f64) {}
    fn on_unload<'a>(&mut self, &'a mut ElementApi, f64) {}
    fn on_tick<'a>(&mut self, &'a mut ElementApi, f64) {}
    fn on_render<'a>(&mut self, &'a mut ElementApi, f64) {}
}
