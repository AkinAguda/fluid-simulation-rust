use percy_dom::*;

use crate::log;

pub struct Canvas {}

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
