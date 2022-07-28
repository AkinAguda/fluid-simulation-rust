use super::super::range::range_view::Range;
use crate::{log, constants as sim_c};
use percy_dom::*;

pub struct ConfigComponentData {
    pub open: bool,
    pub velocity: f32,
    pub time_step: f32,
    pub density: f32,
    pub diffusion: f32
}

pub struct Config {
    pub data: ConfigComponentData
}

impl View for Config {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("config.css");
        let ranges = vec![
            Range {
                key: "dt",
                title: "Time Step",
                value: self.data.time_step,
                min: sim_c::DEFAULT_MIN_TIME_STEP,
                max: sim_c::DEFAULT_MAX_TIME_STEP,
                step: sim_c::DEFAULT_TIME_STEP_STEP,
            },
            Range {
                key: "added_d",
                title: "Added Density",
                value: self.data.density,
                min: sim_c::DEFAULT_ADDED_DENSITY_MIN,
                max: sim_c::DEFAULT_ADDED_DENSITY_MAX,
                step: sim_c::DEFAULT_ADDED_DENSITY_STEP,
            },
            Range {
                key: "added_v",
                title: "Added Velocity",
                value: self.data.velocity,
                min: sim_c::DEFAULT_ADDED_VELOCITY_MIN,
                max: sim_c::DEFAULT_ADDED_VELOCITY_MAX,
                step: sim_c::DEFAULT_ADDED_VELOCITY_STEP,
            },
            Range {
                key: "diff",
                title: "Diffusion",
                value: self.data.diffusion,
                min: sim_c::DEFAULT_MIN_DIFFUSION,
                max: sim_c::DEFAULT_MAX_DIFFUSION,
                step: sim_c::DEFAULT_DIFFUSION_STEP,
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
