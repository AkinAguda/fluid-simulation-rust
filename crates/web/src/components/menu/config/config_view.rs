use std::rc::Rc;

use super::super::range::range_view::Range;
use crate::{
    utility::{constants as sim_c, enums::FluidProperty},
    ClearFluidFn,
};
use percy_dom::*;

pub struct ConfigComponentData {
    pub open: bool,
    pub velocity: f32,
    pub time_step: f32,
    pub density: f32,
    pub diffusion: f32,
    pub set_fluid_property: Rc<dyn Fn(FluidProperty) -> ()>,
    pub clear_fluid: ClearFluidFn,
}

pub struct Config {
    pub data: ConfigComponentData,
}

impl Config {
    fn get_property_updater(&self, f: Rc<dyn Fn(f32) -> FluidProperty>) -> Rc<dyn Fn(f32) -> ()> {
        let set_fluid_property_clone = self.data.set_fluid_property.clone();
        Rc::new(move |val: f32| {
            (set_fluid_property_clone)((*f)(val));
        })
    }
}

impl View for Config {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("config.css");

        let set_time_step = self.get_property_updater(Rc::new(|val| FluidProperty::TimeStep(val)));
        let set_density = self.get_property_updater(Rc::new(|val| FluidProperty::Density(val)));
        let set_velocity = self.get_property_updater(Rc::new(|val| FluidProperty::Velocity(val)));
        let set_diffusion = self.get_property_updater(Rc::new(|val| FluidProperty::Diffusion(val)));

        let clear_fluid = self.data.clear_fluid.clone();

        let ranges = vec![
            Range {
                key: "dt",
                title: "Time Step",
                value: self.data.time_step,
                min: sim_c::DEFAULT_MIN_TIME_STEP,
                max: sim_c::DEFAULT_MAX_TIME_STEP,
                step: sim_c::DEFAULT_TIME_STEP_STEP,
                oninput: set_time_step,
            },
            Range {
                key: "added_d",
                title: "Added Density",
                value: self.data.density,
                min: sim_c::DEFAULT_ADDED_DENSITY_MIN,
                max: sim_c::DEFAULT_ADDED_DENSITY_MAX,
                step: sim_c::DEFAULT_ADDED_DENSITY_STEP,
                oninput: set_density,
            },
            Range {
                key: "added_v",
                title: "Added Velocity",
                value: self.data.velocity,
                min: sim_c::DEFAULT_ADDED_VELOCITY_MIN,
                max: sim_c::DEFAULT_ADDED_VELOCITY_MAX,
                step: sim_c::DEFAULT_ADDED_VELOCITY_STEP,
                oninput: set_velocity,
            },
            Range {
                key: "diff",
                title: "Diffusion",
                value: self.data.diffusion,
                min: sim_c::DEFAULT_MIN_DIFFUSION,
                max: sim_c::DEFAULT_MAX_DIFFUSION,
                step: sim_c::DEFAULT_DIFFUSION_STEP,
                oninput: set_diffusion,
            },
        ];

        let mut config_class = vec![css["config-dropdown"]];

        if self.data.open {
            config_class.push(css["open"])
        } else {
            config_class.push(css["close"])
        }
        html! {
            <ul class=config_class>
                { ranges }
                <li>
                    <button onclick=move || {
                       (clear_fluid)()
                    }
                    class=css["config-button"]>Clear</button>
                </li>
                <li>
                    <button class=css["config-button"]>Reset</button>
                </li>
            </ul>
        }
    }
}
