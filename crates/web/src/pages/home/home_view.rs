use std::rc::Rc;
use percy_dom::*;

use crate::{
    components::{menu::menu_view::MenuData, *},
    world::SimAppWorldWrapper,
};

use crate::world::Msg;
use canvas::canvas_view::Canvas;
use menu::menu_view::Menu;

pub struct Home {
    pub world: SimAppWorldWrapper,
}

impl View for Home {
    fn render(&self) -> VirtualNode {
        let world = self.world.clone();
        let world2 = self.world.clone();
        let open = world.read().state.config_open.clone();
        let css = css_mod::get!("home.css");

        let toggle_config = Rc::new(move || world.msg(Msg::ToggleConfig));

        html! {
        <div class=css["wrapper"]>
            <div class=css["intro"]>
                <h1>Fluid SImulation</h1>
                <h2>With the <a href="https://en.wikipedia.org/wiki/Navier%E2%80%93Stokes_equations" target="_blank">Navier Stokes Equations</a></h2>
                <a href="https://github.com/AkinAguda/fluid-simulation-rust" target="_blank">See on Github</a>
            </div>
            <Menu data={
                MenuData {
                    toggle_config,
                    open: open,
                    config_data: &world2.read().state.config_data
                }
            } />
            <Canvas />
        </div>
        }
    }
}
