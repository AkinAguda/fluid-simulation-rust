use std::rc::Rc;

use percy_dom::{event::MouseEvent, *};

use crate::{
    log,
    utility::{
        constants::CANVAS_ID,
        functions::{get_client_values, InputEvents},
    },
    MouseStateRef,
};

pub struct Canvas {
    pub mouse_state: MouseStateRef,
}

impl View for Canvas {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("canvas.css");

        let mouse_state_ref_1 = self.mouse_state.clone();
        let mouse_state_ref_2 = self.mouse_state.clone();
        let mouse_state_ref_3 = self.mouse_state.clone();

        let onclick: Box<dyn Fn(MouseEvent) -> ()> = Box::new(move |event: MouseEvent| {
            let (client_x, client_y) = get_client_values(InputEvents::Mouse(event));
            mouse_state_ref_1.borrow_mut().is_dragging = true;
        });

        let onmousedown = |event: MouseEvent| {
            mouse_state_ref_2.borrow_mut().mouse_down = true;
        };

        let onmousemove = |event: MouseEvent| {
            if mouse_state_ref_3.borrow().mouse_down {
                mouse_state_ref_2.borrow_mut().mouse_down = true;
                let (client_x, client_y) = get_client_values(InputEvents::Mouse(event));
            }
        };

        // handleEvent = (x: number, y: number, clientX: number, clientY: number) => {
        //     if (this.mode === 0) {
        //       this.addV(x, y, clientX, clientY);
        //       this.addD(x, y);
        //     } else if (this.mode === 1) {
        //       this.addV(x, y, clientX, clientY);
        //     } else if (this.mode === 2) {
        //       this.addD(x, y);
        //     }
        //   };

        html! {
            <canvas
                id=CANVAS_ID
                key=CANVAS_ID
                class=css["canvas"]
                onclick=move |event: MouseEvent| {
                    (onclick)(event);
                }
                onmousedown=move |event: MouseEvent| {
                    (onmousedown)(event);
                }
                onmousemove=move |event: MouseEvent| {
                    (onmousemove)(event);
                }
            >
            /canvas>
        }
    }
}
