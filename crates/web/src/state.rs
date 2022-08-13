use fluid_sim::{Fluid, FluidConfig};

use crate::utility::constants::{
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
        let diffusion = DEFAULT_DIFFUSION;
        let fluid_config = FluidConfig::new(10, 10, diffusion, DEFAULT_TIME_STEP);
        SimAppState {
            config_open: true,
            config_data: ConfigData {
                time_step: DEFAULT_TIME_STEP,
                diffusion,
                density: DEFAULT_ADDED_DENSITY,
                velocity: DEFAULT_ADDED_VELOCITY,
            },
            fluid: Fluid::new(fluid_config),
        }
    }

    pub fn set_config_open(&mut self, state: bool) {
        self.config_open = state;
    }
}
