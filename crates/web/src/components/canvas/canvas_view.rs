use percy_dom::*;
use std::rc::Rc;

use crate::utility::constants::CANVAS_ID;

pub struct Canvas {
    pub set_fluid_size: Rc<dyn Fn(u16, u16) -> ()>,
}

impl View for Canvas {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("canvas.css");
        html! {
            <canvas
                id=CANVAS_ID
                key=CANVAS_ID
                class=css["canvas"]
            ></canvas>
        }
    }
}
