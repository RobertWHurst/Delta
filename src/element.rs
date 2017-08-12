use std::cmp;
use std::fmt;
use position::Position;
use sprite::Sprite;
use controller::Controller;
use velocity::Velocity;
use controller::Api;
use time::PreciseTime;

pub struct Element {
    pub id: String,
    pub position: Position,
    pub velocity: Velocity,
    pub sprite: Option<Sprite>,
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
            position: Position::default(),
            velocity: Velocity::default(),
            sprite: None,
            controller: box controller,
            last_tick_at: None,
        }
    }
    pub fn tick(&mut self, mut api: Api) {
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

        api.set_tick_delta(delta_time_seconds);

        self.controller.tick(api);
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
            "Element {{ id: {} position: {:?}, velocity: {:?} }}",
            self.id,
            self.position,
            self.velocity,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct BoxController {}
    impl Controller for BoxController {}

    #[test]
    fn create_element() {
        let box_controller = BoxController {};

        Element::new("box", box_controller);
    }
}
