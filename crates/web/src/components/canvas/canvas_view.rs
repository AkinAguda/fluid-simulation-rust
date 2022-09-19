use std::ops::Deref;

use percy_dom::{event::MouseEvent, *};

use crate::{
    log,
    utility::{
        constants::CANVAS_ID,
        functions::{get_client_values, InputEvents},
        structs::MouseState,
    },
    AddPropertiesFn, MouseStateRef,
};

pub struct CanvasData {
    pub mouse_state: MouseStateRef,
    pub add_properties_from_mouse_loc: AddPropertiesFn,
}

pub struct Canvas {
    pub data: CanvasData,
}

impl View for Canvas {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("canvas.css");

        let mouse_state_ref_1 = self.data.mouse_state.clone();
        let mouse_state_ref_2 = self.data.mouse_state.clone();
        let mouse_state_ref_3 = self.data.mouse_state.clone();

        let add_properties_from_mouse_loc_ref_1 = self.data.add_properties_from_mouse_loc.clone();
        let add_properties_from_mouse_loc_ref_2 = self.data.add_properties_from_mouse_loc.clone();

        let onclick = move |event: web_sys::MouseEvent| {
            (add_properties_from_mouse_loc_ref_1)(get_client_values(InputEvents::Mouse(event)));
        };

        let onmousedown = move || {
            mouse_state_ref_2.borrow_mut().mouse_down = true;
        };

        let onmouseup = move || {
            mouse_state_ref_1.borrow_mut().reset();
        };

        let onmousemove = move |event: web_sys::MouseEvent| {
            let md = mouse_state_ref_3.borrow().mouse_down;
            if md {
                mouse_state_ref_3.borrow_mut().mouse_down = true;
                (add_properties_from_mouse_loc_ref_2)(get_client_values(InputEvents::Mouse(event)));
            }
        };

        html! {
            <canvas
                id=CANVAS_ID
                key=CANVAS_ID
                class=css["canvas"]
                onclick=move |event: MouseEvent| {
                    (onclick)(event.deref().clone());
                }
                onmousedown=move || {
                    (onmousedown)();
                }
                onmousemove=move |event: web_sys::MouseEvent| {
                    (onmousemove)(event);
                }
                onmouseup=move || {
                    (onmouseup)();
                }
            >
            /canvas>
        }
    }
}
