use crate::{
    resources::{RenderFn, Resources},
    state::SimAppState,
};
use app_world::{AppWorld, AppWorldWrapper};

use crate::log;
pub struct World {
    pub state: SimAppState,
    resources: Resources,
}

pub struct WorldConfig {
    pub render_fn: RenderFn,
}

pub type SimAppWorldWrapper = AppWorldWrapper<World>;

pub enum Msg {
    ToggleConfig,
    SetBtnTxt(String),
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
        // world_wrapper.read().state.config_open
        self.state.set_config_open(false);
    }
}

impl AppWorld for World {
    type Message = Msg;

    fn msg(&mut self, message: Self::Message) {
        match message {
            Msg::ToggleConfig => {
                log("IN APP reducer");
                self.toggle_config();
            }
            Msg::SetBtnTxt(text) => self.state.set_btn_txt(text),
        }
        (self.resources.render_fn)();
    }
}

pub(crate) fn create_world(config: WorldConfig) -> World {
    World {
        state: SimAppState::new(),
        resources: Resources {
            render_fn: config.render_fn,
        },
    }
}
