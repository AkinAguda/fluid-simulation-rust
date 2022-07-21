pub struct SimAppState {
    pub config_open: bool,
}

impl SimAppState {
    pub fn new() -> SimAppState {
        SimAppState {
            config_open: true,
        }
    }

    pub fn set_config_open(&mut self, state: bool) {
        self.config_open = state;
    }

}
