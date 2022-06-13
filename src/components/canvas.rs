use percy_dom::*;

pub struct Canvas {}

impl View for Canvas {
    fn render(&self) -> VirtualNode {
        html! {
            <canvas class="canvas"></canvas>
        }
    }
}
