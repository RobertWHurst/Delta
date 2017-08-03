use super::Element;
use super::super::Scene;


pub trait Controller {
    // TODO: Controller API
    fn on_load<C>(&mut self, f64) where C: Controller {}
    fn on_unload<C>(&mut self, f64) where C: Controller {}
    fn on_tick<C>(&mut self, f64) where C: Controller {}
    fn on_render<C>(&mut self, f64) where C: Controller {}
}
