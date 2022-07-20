use std::rc::Rc;

use app_world::AppWorldWrapper;
use percy_dom::*;

use crate::{
    components::*,
    world::{Msg, SimAppWorldWrapper, World},
};

use canvas::canvas_view::Canvas;
use menu::menu_view::Menu;

// pub fn set_w() {

// };

pub struct Home {
}

impl View for Home {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("home.css");
        // let msg_handler = self.msg_say_hello.clone();

        html! {
        <div class=css["wrapper"]>
            <div class=css["intro"]>
                <h1>Fluid SImulation</h1>
                <h2>With the <a href="https://en.wikipedia.org/wiki/Navier%E2%80%93Stokes_equations" target="_blank">Navier Stokes Equations</a></h2>
                <a href="https://github.com/AkinAguda/fluid-simulation-rust" target="_blank">See on Github</a>
            </div>
            // <Menu world={self.world} />
            <Canvas />
            // <button onclick=move|| {
            //     (msg_handler)("Hello world".to_string())
            // }>{self.button_text.clone()}</button>
        </div>
        }
    }
}
