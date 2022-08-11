use std::rc::Rc;

use percy_dom::*;

use crate::log;

pub struct Canvas {
    pub set_fluid_size: Rc<dyn Fn(u16, u16) -> ()>,
}

impl View for Canvas {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("canvas.css");
        html! {
            <canvas
                key="main-canvas"
                class=css["canvas"]
                on_create_element = move |element: web_sys::Element| {
                    log("CREATED");
                }
            ></canvas>
        }
    }
}
