use std::rc::Rc;

use percy_dom::*;

use crate::utility::functions::{get_display_dimensions, resize_canvas_to_display_size};

pub struct Canvas {
    pub set_fluid_size: Rc<dyn Fn(u16, u16) -> ()>,
}

impl View for Canvas {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("canvas.css");
        let set_fluid_size = self.set_fluid_size.clone();
        html! {
            <canvas
                id="main-canvas"
                key="main-canvas"
                class=css["canvas"]
                // on_create_element = move |canvas: web_sys::Element| {
                //     let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
                //     resize_canvas_to_display_size(&canvas);
                //     let (width, height) = get_display_dimensions(canvas.width(), canvas.height());
                //     (set_fluid_size)(width as u16, height as u16);
                // }
            ></canvas>
        }
    }
}
