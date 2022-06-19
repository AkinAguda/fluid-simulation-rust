use percy_dom::*;

pub struct Canvas {}

impl View for Canvas {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("canvas.css");
        html! {
            <canvas class=css["canvas"]></canvas>
        }
    }
}
