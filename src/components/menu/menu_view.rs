use app_world::AppWorldWrapper;
use percy_dom::*;

use crate::world::World;

use super::{hamburger::hamburger_view::render_hamburger, *};
use config::config_view::Config;

pub struct Menu {
    pub world: AppWorldWrapper<World>,
}

impl View for Menu {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("menu.css");
        html! {
            <div class=css["config-wrapper"]>

                // <Hamburger world={self.world} />
                // {render_hamburger(self.world)}
                // <Config world={self.world} />
            </div>
        }
    }
}
