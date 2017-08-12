use std::cmp;
use std::fmt;
use std::sync::{Arc, Mutex};
use position::Position;
use sprite::Sprite;
use controller::Controller;
use velocity::Velocity;
use controller::ControllerApi;
use time::PreciseTime;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ElementState {
    pub position: Position,
    pub velocity: Velocity,
    pub sprite: Option<Sprite>,
}

pub struct Element {
    id: String,
    state_mx: Arc<Mutex<ElementState>>,
    controller: Box<Controller + Send>,
    last_tick_at: Option<PreciseTime>,
}

impl Element {
    pub fn new<S, C>(id: S, controller: C) -> Self
    where
        S: Into<String>,
        C: 'static + Controller + Send,
    {
        Self {
            id: id.into(),
            state_mx: Arc::new(Mutex::new(ElementState::default())),
            controller: box controller,
            last_tick_at: None,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn controller(&mut self) -> &mut Box<Controller + Send> {
        &mut self.controller
    }

    pub fn api(&self) -> ElementApi {
        ElementApi::new(self.id(), ElementData::ElementState(self.state_mx.clone()))
    }

    pub fn tick(&mut self, mut controller_api: ControllerApi) {
        let tick_at = PreciseTime::now();

        let last_tick_at = match self.last_tick_at {
            Some(t) => t,
            None => {
                self.last_tick_at = Some(tick_at);
                return;
            }
        };
        self.last_tick_at = Some(tick_at);

        let tick_duration = last_tick_at.to(tick_at);

        let delta_time_nanos = tick_duration.num_nanoseconds().unwrap_or(i64::max_value());
        let delta_time_seconds = delta_time_nanos as f64 / 1000. / 1000. / 1000.;

        controller_api.set_tick_delta(delta_time_seconds);

        let element_api = self.api();

        self.controller.tick(element_api, controller_api);
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Element {}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Element {{ id: {} state: {:?} }}",
            self.id,
            self.state_mx,
        )
    }
}

#[derive(Debug)]
pub enum ElementData {
    Element(Arc<Mutex<Element>>),
    ElementState(Arc<Mutex<ElementState>>),
}

#[derive(Debug)]
pub struct ElementApi {
    id: String,
    data: ElementData,
}

impl ElementApi {
    pub fn new<S>(into_id: S, data: ElementData) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: into_id.into(),
            data,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn state(&self) -> Arc<Mutex<ElementState>> {
        match &self.data {
            &ElementData::Element(ref e) => e.lock().unwrap().state_mx.clone(),
            &ElementData::ElementState(ref s) => s.clone(),
        }
    }

    pub fn position(&self) -> Position {
        let state_mx = self.state();
        let state = state_mx.lock().unwrap();
        state.position
    }
}
