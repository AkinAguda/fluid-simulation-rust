use percy_dom::*;

use crate::components::*;

pub struct Config {}

impl View for Config {
    fn render(&self) -> VirtualNode {
        html! {
            <div class="config-wrapper">

            </div>
        }
    }
}
