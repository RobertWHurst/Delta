use scene::SceneRenderEntity;

pub struct ElementBuilder {
    pub id       : &'static str,
    pub width    : Option<usize>,
    pub height   : Option<usize>,
    pub x        : Option<f64>,
    pub y        : Option<f64>,
    pub z        : Option<f64>,
    pub on_tick  : Option<fn(Element, f64)>,
    pub on_render: Option<fn(Element, f64)>,
}

impl ElementBuilder {
    pub fn new(id: &'static str) -> ElementBuilder {
        ElementBuilder {
            id       : id,
            width    : None,
            height   : None,
            x        : None,
            y        : None,
            z        : None,
            on_tick  : None,
            on_render: None,
        }
    }
    pub fn width(mut self, width: usize) -> ElementBuilder {
        self.width = Some(width); self
    }
    pub fn height(mut self, height: usize) -> ElementBuilder {
        self.height = Some(height); self
    }
    pub fn x(mut self, x: f64) -> ElementBuilder {
        self.x = Some(x); self
    }
    pub fn y(mut self, y: f64) -> ElementBuilder {
        self.y = Some(y); self
    }
    pub fn z(mut self, z: f64) -> ElementBuilder {
        self.z = Some(z); self
    }
    pub fn on_tick(mut self, on_tick: fn(Element, f64)) -> ElementBuilder {
        self.on_tick = Some(on_tick); self
    }
    pub fn on_render(mut self, on_render: fn(Element, f64)) -> ElementBuilder {
        self.on_render = Some(on_render); self
    }
    pub fn build(self) -> Element {
        Element {
            id       : self.id,
            width    : self.width.unwrap_or(0),
            height   : self.height.unwrap_or(0),
            x        : self.x.unwrap_or(0.),
            y        : self.y.unwrap_or(0.),
            z        : self.z.unwrap_or(0.),
            on_tick  : self.on_tick.unwrap_or(|_, _|{}),
            on_render: self.on_render.unwrap_or(|_, _|{}),
        }
    }
}

#[derive(Debug)]
pub struct Element {
    pub id    : &'static str,
    pub width : usize,
    pub height: usize,
    pub x     : f64,
    pub y     : f64,
    pub z     : f64,
    on_tick   : fn(Element, f64),
    on_render : fn(Element, f64),
}

impl Element {
    pub fn render(&self) -> SceneRenderEntity {
        SceneRenderEntity::Sprite(self.id)
    }
}
