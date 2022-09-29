use crate::utility::structs::ConfigData;

pub struct SimAppState {
    pub config_open: bool,
    pub config_data: ConfigData,
}

impl SimAppState {
    pub fn new() -> SimAppState {
        SimAppState {
            config_open: true,
            config_data: ConfigData::default(),
        }
    }

    pub fn set_config_open(&mut self, state: bool) {
        self.config_open = state;
    }
}
