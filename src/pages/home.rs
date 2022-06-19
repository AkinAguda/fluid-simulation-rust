use percy_dom::*;

use crate::components::*;

use canvas::Canvas;
use menu::config::config_view::Config;

pub struct Home {}

impl View for Home {
    fn render(&self) -> VirtualNode {
        html! {
        <div class="wrapper">
            <div class="intro">
                <h1 class="heading">Fluid SImulation</h1>
                <h2>With the <a href="https://en.wikipedia.org/wiki/Navier%E2%80%93Stokes_equations" target="_blank">Navier Stokes Equations</a></h2>
                <a href="https://github.com/AkinAguda/fluid-simulation-rust" target="_blank">See on Github</a>
            </div>
            <Config />
            <Canvas />
        </div>
        }
    }
}
