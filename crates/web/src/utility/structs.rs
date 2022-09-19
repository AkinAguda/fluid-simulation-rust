use wasm_bindgen::closure::Closure;

pub struct ConfigData {
    pub time_step: f32,
    pub diffusion: f32,
    pub density: f32,
    pub velocity: f32,
}

pub struct RenderLoop {
    pub animation_id: Option<i32>,
    pub closure: Option<Closure<dyn Fn()>>,
}

impl RenderLoop {
    pub fn new(animation_id: Option<i32>, closure: Option<Closure<dyn Fn()>>) -> RenderLoop {
        RenderLoop {
            animation_id,
            closure,
        }
    }
}

pub struct MouseState {
    pub mouse_down: bool,
    pub is_dragging: bool,
    pub pos: (f64, f64),
}

impl Default for MouseState {
    fn default() -> Self {
        MouseState {
            mouse_down: false,
            is_dragging: false,
            pos: (0.0, 0.0),
        }
    }
}

impl MouseState {
    pub fn reset(&mut self) {
        self.mouse_down = false;
        self.is_dragging = false;
        self.pos = (0.0, 0.0);
    }
}
