use fluid_sim::{Fluid, FluidConfig};

use crate::constants::{
    DEFAULT_ADDED_DENSITY, DEFAULT_ADDED_VELOCITY, DEFAULT_DIFFUSION, DEFAULT_TIME_STEP,
};
use crate::utility::structs::ConfigData;

pub struct SimAppState {
    pub config_open: bool,
    pub config_data: ConfigData,
    pub fluid: Fluid,
}

impl SimAppState {
    pub fn new() -> SimAppState {
        let fluid_config = FluidConfig::new(10, 10, DEFAULT_DIFFUSION);
        SimAppState {
            config_open: true,
            config_data: ConfigData {
                time_step: DEFAULT_TIME_STEP,
                diffusion: DEFAULT_DIFFUSION,
                density: DEFAULT_ADDED_DENSITY,
                velocity: DEFAULT_ADDED_VELOCITY,
            },
            fluid: Fluid::new(fluid_config, DEFAULT_TIME_STEP),
        }
    }

    pub fn set_config_open(&mut self, state: bool) {
        self.config_open = state;
    }
}
