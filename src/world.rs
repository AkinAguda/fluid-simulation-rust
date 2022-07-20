use crate::{
    resources::{RenderFn, Resources},
    state::SimAppState,
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
        }
        (self.resources.render_fn)();
    }
}
