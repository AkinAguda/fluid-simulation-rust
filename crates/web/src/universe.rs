use crate::{
    resources::{FluidProperySetters, Resources},
    state::SimAppState,
    utility::enums::FluidProperty,
};
use app_universe::{AppUniverse, AppUniverseCore};

pub struct AppState {
    pub state: SimAppState,
    resources: Resources,
}

pub type SimAppUniverseWrapper = AppUniverse<AppState>;

pub enum Msg {
    ToggleConfig,
    SetFluidProperty(FluidProperty),
    SetFluidPropertySetters(FluidProperySetters),
    ResetConfig,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            state: SimAppState::new(),
            resources: Resources {
                fluid_propery_setters: FluidProperySetters {
                    diffusion: Box::new(|_val: f32| {}),
                    time_step: Box::new(|_val: f32| {}),
                },
            },
        }
    }

    pub fn toggle_config(&mut self) {
        self.state.set_config_open(!self.state.config_open);
    }
}

impl AppUniverseCore for AppState {
    type Message = Msg;

    fn msg(&mut self, message: Self::Message) {
        match message {
            Msg::ToggleConfig => {
                self.toggle_config();
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
    }
}
