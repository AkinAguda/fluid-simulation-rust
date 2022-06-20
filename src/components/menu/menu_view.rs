use percy_dom::*;

use super::*;
use config::config_view::Config;
use hamburger::hamburger_view::Hamburger;
pub struct Menu {}

impl View for Menu {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("menu.css");

        html! {
            <div class=css["config-wrapper"]>
                <Hamburger />
                <Config />
            </div>
        }
    }
}
