use std::rc::Rc;

use percy_dom::*;

use crate::{
    components::menu::config,
    utility::{enums::FluidProperty, structs::ConfigData},
    ClearFluidFn,
};

use super::hamburger::hamburger_view::Hamburger;
use config::config_view::{Config, ConfigComponentData};

pub struct MenuData<'a> {
    pub toggle_config: Rc<dyn Fn() -> ()>,
    pub set_fluid_property: Rc<dyn Fn(FluidProperty) -> ()>,
    pub config_data: &'a ConfigData,
    pub open: bool,
    pub clear_fluid: ClearFluidFn,
}

pub struct Menu<'a> {
    pub data: MenuData<'a>,
}

impl<'a> View for Menu<'a> {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("menu.css");
        html! {
            <div class=css["config-wrapper"]>

                <Hamburger toggle_config={self.data.toggle_config.clone()} />

                <Config
                    data={
                        ConfigComponentData {
                            open: self.data.open,
                            velocity: self.data.config_data.velocity,
                            time_step: self.data.config_data.time_step,
                            density: self.data.config_data.density,
                            diffusion: self.data.config_data.diffusion,
                            set_fluid_property: self.data.set_fluid_property.clone(),
                            clear_fluid: self.data.clear_fluid.clone()
                        }
                    }
                />
            </div>
        }
    }
}
