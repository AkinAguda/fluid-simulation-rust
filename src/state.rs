pub struct SimAppState {
    pub config_open: bool,
    pub btn_txt: String,
}

impl SimAppState {
    pub fn new() -> SimAppState {
        SimAppState {
            config_open: true,
            btn_txt: String::from("hello"),
        }
    }

    pub fn set_config_open(&mut self, state: bool) {
        self.config_open = state;
    }

    pub fn set_btn_txt(&mut self, value: String) {
        self.btn_txt = value;
    }
}
