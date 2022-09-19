use percy_dom::*;
use std::rc::Rc;

use crate::{
    components::{canvas::canvas_view::CanvasData, menu::menu_view::MenuData, *},
    utility::{enums::FluidProperty, functions::wrld_clbk},
    world::SimAppWorldWrapper,
    AddPropertiesFn, MouseStateRef,
};

use crate::world::Msg;
use canvas::canvas_view::Canvas;
use menu::menu_view::Menu;

pub struct Home {
    pub world: SimAppWorldWrapper,
    pub mouse_state: MouseStateRef,
    pub add_properties_from_mouse_loc: AddPropertiesFn,
}

impl View for Home {
    fn render(&self) -> VirtualNode {
        let open = self.world.read().state.config_open.clone();
        let css = css_mod::get!("home.css");

        let toggle_config = wrld_clbk(&self.world, |world| {
            Rc::new(move || world.msg(Msg::ToggleConfig))
        });

        let set_fluid_property = wrld_clbk(&self.world, |world| {
            Rc::new(move |property: FluidProperty| world.msg(Msg::SetFluidProperty(property)))
        });

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
                    config_data: &self.world.read().state.config_data,
                }
            } />

            <Canvas
                data={
                    CanvasData {
                        mouse_state: self.mouse_state.clone(),
                        add_properties_from_mouse_loc: self.add_properties_from_mouse_loc.clone()
                    }
                }
            />
        </div>
        }
    }
}
