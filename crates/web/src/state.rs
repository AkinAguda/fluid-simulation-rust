use crate::utility::constants::{
    DEFAULT_ADDED_DENSITY, DEFAULT_ADDED_VELOCITY, DEFAULT_DIFFUSION, DEFAULT_TIME_STEP,
};
use crate::utility::structs::ConfigData;

pub struct SimAppState {
    pub config_open: bool,
    pub config_data: ConfigData,
}

impl SimAppState {
    pub fn new() -> SimAppState {
        SimAppState {
            config_open: true,
            config_data: ConfigData {
                time_step: DEFAULT_TIME_STEP,
                diffusion: DEFAULT_DIFFUSION,
                density: DEFAULT_ADDED_DENSITY,
                velocity: DEFAULT_ADDED_VELOCITY,
            },
        }
    }

    pub fn set_config_open(&mut self, state: bool) {
        self.config_open = state;
    }
}
