use percy_dom::*;
use std::rc::Rc;

use crate::{
    components::{canvas::canvas_view::CanvasData, menu::menu_view::MenuData, *},
    utility::{enums::FluidProperty, functions::wrld_clbk},
    world::SimAppWorldWrapper,
    AddPropertiesFn, ClearFluidFn, MouseStateRef,
};

use crate::world::Msg;
use canvas::canvas_view::Canvas;
use menu::menu_view::Menu;

pub struct Home {
    pub world: SimAppWorldWrapper,
    pub mouse_state: MouseStateRef,
    pub add_properties_from_mouse_loc: AddPropertiesFn,
    pub clear_fluid: ClearFluidFn,
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

        let reset_config = wrld_clbk(&self.world, |world| {
            Rc::new(move || world.msg(Msg::ResetConfig))
        });

        let main_heading = "Fluid SImulation";
        let sub_heading_start = "With the ";
        let sub_heading = "Navier Stokes Equations";
        let cta_link_text = "See on Github";

        html! {
        <div class=css["wrapper"]>
            <div class=css["intro"]>
                <h1>{main_heading}</h1>
                <h2>{sub_heading_start}<a href="https://en.wikipedia.org/wiki/Navier%E2%80%93Stokes_equations" target="_blank">{sub_heading}</a></h2>
                <a href="https://github.com/AkinAguda/fluid-simulation-rust" target="_blank">{cta_link_text}</a>
            </div>
            <Menu data={
                MenuData {
                    open: open,
                    toggle_config,
                    set_fluid_property,
                    config_data: &self.world.read().state.config_data,
                    clear_fluid: self.clear_fluid.clone(),
                    reset_config: reset_config
                }
            } />

            <Canvas
                data={
                    CanvasData {
                        mouse_state: self.mouse_state.clone(),
                        add_properties_from_mouse_loc: self.add_properties_from_mouse_loc.clone(),
                    }
                }
            />
        </div>
        }
    }
}
