use std::rc::Rc;

use percy_dom::*;

use crate::components::menu::config;

use super::hamburger::hamburger_view::Hamburger;
use config::config_view::Config;

pub struct MenuData {
    pub toggle_config: Rc<dyn Fn() -> ()>,
    pub open: bool,
}

pub struct Menu {
    pub data: MenuData,
}

impl View for Menu {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("menu.css");
        html! {
            <div class=css["config-wrapper"]>

                <Hamburger toggle_config={self.data.toggle_config.clone()} />

                <Config open={self.data.open} />
            </div>
        }
    }
}
