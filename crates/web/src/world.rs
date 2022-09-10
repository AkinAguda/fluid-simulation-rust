use crate::{
    resources::{RenderFn, Resources},
    state::SimAppState,
    utility::enums::FluidProperty,
};
use app_world::{AppWorld, AppWorldWrapper};

pub struct World {
    pub state: SimAppState,
    resources: Resources,
}

pub type SimAppWorldWrapper = AppWorldWrapper<World>;

pub enum Msg {
    ToggleConfig,
    SetRenderFn(RenderFn),
    SetFluidProperty(FluidProperty),
    // UpdateFluidSize(u16, u16),
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
                FluidProperty::Diffusion(value) => {
                    self.state.config_data.diffusion = value;
                    // self.state.fluid.config.set_diffusion(value);
                }
                FluidProperty::TimeStep(value) => {
                    self.state.config_data.time_step = value;
                    // self.state.fluid.config.dt = value;
                }
                FluidProperty::Density(value) => self.state.config_data.density = value,
                FluidProperty::Velocity(value) => self.state.config_data.velocity = value,
            },
            // Msg::UpdateFluidSize(nw, nh) => {
            //     self.state.fluid.config.update_Size(nw, nh);
            // }
        }
        (self.resources.render_fn)();
    }
}
