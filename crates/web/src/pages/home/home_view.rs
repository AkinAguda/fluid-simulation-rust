use percy_dom::*;
use std::rc::Rc;

use crate::{
    components::{menu::menu_view::MenuData, *},
    utility::enums::FluidProperty,
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
        let world3 = self.world.clone();
        let open = world.read().state.config_open.clone();
        let css = css_mod::get!("home.css");

        let toggle_config = Rc::new(move || world.msg(Msg::ToggleConfig));
        let set_fluid_property =
            Rc::new(move |property: FluidProperty| world3.msg(Msg::SetFluidProperty(property)));

        html! {
        <div class=css["wrapper"]>
            <div class=css["intro"]>
                <h1>Fluid SImulation</h1>
                <h2>With the <a href="https://en.wikipedia.org/wiki/Navier%E2%80%93Stokes_equations" target="_blank">Navier Stokes Equations</a></h2>
                <a href="https://github.com/AkinAguda/fluid-simulation-rust" target="_blank">See on Github</a>
            </div>
            <Menu data={
                MenuData {
                    open: open,
                    toggle_config,
                    set_fluid_property,
                    config_data: &world2.read().state.config_data,
                }
            } />
            <Canvas />
        </div>
        }
    }
}
