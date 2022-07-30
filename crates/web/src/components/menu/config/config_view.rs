use std::rc::Rc;

use super::super::range::range_view::Range;
use crate::{ constants as sim_c, utility::enums::FluidProperty};
use percy_dom::*;

pub struct ConfigComponentData {
    pub open: bool,
    pub velocity: f32,
    pub time_step: f32,
    pub density: f32,
    pub diffusion: f32,
    pub set_fluid_property: Rc<dyn Fn(FluidProperty) -> ()>,
}

pub struct Config {
    pub data: ConfigComponentData
}

impl View for Config {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("config.css");
        let set_fluid_property_1 = self.data.set_fluid_property.clone();
        let set_fluid_property_2 = self.data.set_fluid_property.clone();
        let set_fluid_property_3 = self.data.set_fluid_property.clone();
        let set_fluid_property_4 = self.data.set_fluid_property.clone();

        let set_time_step = move |val: f32| {
            (set_fluid_property_1.clone())(FluidProperty::TimeStep(val));
        };
        let set_density = move |val: f32| {
            (set_fluid_property_2.clone())(FluidProperty::Density(val));
        };
        let set_velocity = move |val: f32| {
            (set_fluid_property_3.clone())(FluidProperty::Velocity(val));
        };
        let set_diffusion = move |val: f32| {
            (set_fluid_property_4.clone())(FluidProperty::Diffusion(val));
        };
        let ranges = vec![
            Range {
                key: "dt",
                title: "Time Step",
                value: self.data.time_step,
                min: sim_c::DEFAULT_MIN_TIME_STEP,
                max: sim_c::DEFAULT_MAX_TIME_STEP,
                step: sim_c::DEFAULT_TIME_STEP_STEP,
                oninput: Rc::new(set_time_step)
            },
            Range {
                key: "added_d",
                title: "Added Density",
                value: self.data.density,
                min: sim_c::DEFAULT_ADDED_DENSITY_MIN,
                max: sim_c::DEFAULT_ADDED_DENSITY_MAX,
                step: sim_c::DEFAULT_ADDED_DENSITY_STEP,
                oninput: Rc::new(set_density)
            },
            Range {
                key: "added_v",
                title: "Added Velocity",
                value: self.data.velocity,
                min: sim_c::DEFAULT_ADDED_VELOCITY_MIN,
                max: sim_c::DEFAULT_ADDED_VELOCITY_MAX,
                step: sim_c::DEFAULT_ADDED_VELOCITY_STEP,
                oninput: Rc::new(set_velocity)
            },
            Range {
                key: "diff",
                title: "Diffusion",
                value: self.data.diffusion,
                min: sim_c::DEFAULT_MIN_DIFFUSION,
                max: sim_c::DEFAULT_MAX_DIFFUSION,
                step: sim_c::DEFAULT_DIFFUSION_STEP,
                oninput: Rc::new(set_diffusion)
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
                    <button class=css["config-button"]>Clear</button>
                </li>
                <li>
                    <button class=css["config-button"]>Reset</button>
                </li>
            </ul>
        }
    }
}
