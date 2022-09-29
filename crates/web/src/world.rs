use crate::{
    resources::{FluidProperySetters, RenderFn, Resources},
    state::SimAppState,
    utility::enums::FluidProperty,
};
use app_world::{AppWorld, AppWorldWrapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

pub struct World {
    pub state: SimAppState,
    resources: Resources,
}

pub type SimAppWorldWrapper = AppWorldWrapper<World>;

pub enum Msg {
    ToggleConfig,
    SetRenderFn(RenderFn),
    SetFluidProperty(FluidProperty),
    SetFluidPropertySetters(FluidProperySetters),
    ResetConfig,
}

impl World {
    pub fn new(render_fn: RenderFn) -> World {
        World {
            state: SimAppState::new(),
            resources: Resources {
                render_fn,
                fluid_propery_setters: FluidProperySetters {
                    diffusion: Box::new(|_val: f32| {}),
                    time_step: Box::new(|_val: f32| {}),
                },
            },
        }
    }
    pub fn set_render_fn(&mut self, render_fn: RenderFn) {
        self.resources.set_render_fn(render_fn);
    }
    pub fn toggle_config(&mut self) {
        self.state.set_config_open(!self.state.config_open);
    }
}

impl AppWorld for World {
    type Message = Msg;

    fn msg(&mut self, message: Self::Message) {
        match message {
            Msg::ToggleConfig => {
                self.toggle_config();
            }

            Msg::SetRenderFn(render_fn) => {
                self.set_render_fn(render_fn);
            }

            Msg::SetFluidProperty(fluid_prop) => match fluid_prop {
                FluidProperty::Diffusion(value) => {
                    self.state.config_data.diffusion = value;
                    (self.resources.fluid_propery_setters.diffusion)(value);
                }
                FluidProperty::TimeStep(value) => {
                    self.state.config_data.time_step = value;
                    (self.resources.fluid_propery_setters.time_step)(value);
                }
                FluidProperty::Density(value) => {
                    self.state.config_data.density = value;
                }
                FluidProperty::Velocity(value) => {
                    self.state.config_data.velocity = value;
                }
            },

            Msg::SetFluidPropertySetters(fluid_propery_setters) => {
                self.resources
                    .set_fluid_propery_setters(fluid_propery_setters);
            }

            Msg::ResetConfig => {
                self.state.config_data.reset();
            }
        }
        (self.resources.render_fn)();
    }
}
