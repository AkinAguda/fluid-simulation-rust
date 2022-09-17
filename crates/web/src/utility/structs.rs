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
