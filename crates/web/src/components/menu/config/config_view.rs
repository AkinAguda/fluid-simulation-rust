use super::super::range::range_view::Range;
use crate::{log, constants as sim_c};
use percy_dom::*;
pub struct Config {
    pub open: bool,
}

impl View for Config {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("config.css");
        let ranges = vec![
            Range {
                key: "dt",
                title: "Time Step",
                value: sim_c::DEFAULT_TIME_STEP,
                min: sim_c::DEFAULT_MIN_TIME_STEP,
                max: sim_c::DEFAULT_MAX_TIME_STEP,
                step: sim_c::DEFAULT_TIME_STEP_STEP,
            },
            Range {
                key: "added_d",
                title: "Added Density",
                value: sim_c::DEFAULT_ADDED_DENSITY,
                min: sim_c::DEFAULT_ADDED_DENSITY_MIN,
                max: sim_c::DEFAULT_ADDED_DENSITY_MAX,
                step: sim_c::DEFAULT_ADDED_DENSITY_STEP,
            },
            Range {
                key: "added_v",
                title: "Added Velocity",
                value: sim_c::DEFAULT_ADDED_VELOCITY,
                min: sim_c::DEFAULT_ADDED_VELOCITY_MIN,
                max: sim_c::DEFAULT_ADDED_VELOCITY_MAX,
                step: sim_c::DEFAULT_ADDED_VELOCITY_STEP,
            },
            Range {
                key: "diff",
                title: "Diffusion",
                value: sim_c::DEFAULT_DIFFUSION,
                min: sim_c::DEFAULT_MIN_DIFFUSION,
                max: sim_c::DEFAULT_MAX_DIFFUSION,
                step: sim_c::DEFAULT_DIFFUSION_STEP,
            },
        ];

        let mut config_class = vec![css["config-dropdown"]];

        if self.open {
            log("IS OPEN");
            config_class.push(css["open"])
        } else {
            log("IS CLOSED");
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
