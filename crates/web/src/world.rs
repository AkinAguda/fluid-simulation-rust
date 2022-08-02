use crate::{
    resources::{RenderFn, Resources},
    state::SimAppState,
    utility::enums::FluidProperty,
};
use app_world::{AppWorld, AppWorldWrapper};

use fluid_sim::{Fluid, FluidConfig};

use crate::constants::{DEFAULT_DIFFUSION, DEFAULT_TIME_STEP};

pub struct World {
    pub state: SimAppState,
    resources: Resources,
}

pub type SimAppWorldWrapper = AppWorldWrapper<World>;

pub enum Msg {
    ToggleConfig,
    SetRenderFn(RenderFn),
    SetFluidProperty(FluidProperty),
    CreateFluid(u16, u16),
}

impl World {
    pub fn new(render_fn: RenderFn) -> World {
        World {
            state: SimAppState::new(),
            resources: Resources { render_fn },
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
                FluidProperty::Diffusion(value) => self.state.config_data.diffusion = value,
                FluidProperty::TimeStep(value) => self.state.config_data.time_step = value,
                FluidProperty::Density(value) => self.state.config_data.density = value,
                FluidProperty::Velocity(value) => self.state.config_data.velocity = value,
            },

            Msg::CreateFluid(nw, nh) => {
                let fluid_config = FluidConfig::new(nw, nh, DEFAULT_DIFFUSION);
                self.state.fluid = Fluid::new(fluid_config, DEFAULT_TIME_STEP);
            }
        }
        (self.resources.render_fn)();
    }
}
