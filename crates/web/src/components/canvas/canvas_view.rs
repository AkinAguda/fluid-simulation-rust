use percy_dom::*;

use crate::utility::constants::CANVAS_ID;

pub struct Canvas {}

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
