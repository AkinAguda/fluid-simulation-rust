use super::structs::RenderLoop;
use super::webgl::render_fluid;
use super::webgl::structs::WebGlData;
use crate::universe::SimAppUniverseWrapper;
use crate::utility::constants::CANVAS_ID;
use fluid_sim::Fluid;
use num_traits::ToPrimitive;
use percy_dom::JsCast;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{DomRect, MouseEvent, TouchEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

fn resize_canvas_to_display_size(canvas: &web_sys::HtmlCanvasElement) -> bool {
    let window = web_sys::window().unwrap();
    let dpr = window.device_pixel_ratio();
    let dom_rect = canvas.get_bounding_client_rect();
    let display_width = (dom_rect.width() * dpr).round();
    let display_height = (dom_rect.height() * dpr).round();

    let need_resize = canvas.width().to_f64().unwrap() != display_width
        || canvas.height().to_f64().unwrap() != display_height;

    if need_resize {
        canvas.set_width(display_width.to_u32().unwrap());
        canvas.set_height(display_height.to_u32().unwrap());
    }

    need_resize
}

pub fn get_multipliers(x1: f64, y1: f64, x2: f64, y2: f64) -> (i8, i8) {
    let mut multipliers: (i8, i8) = (0, 0);
    if x2 - x1 > 0.0 {
        multipliers.0 = 1;
    }

    if x2 - x1 < 0.0 {
        multipliers.0 = -1;
    }

    if y2 - y1 > 0.0 {
        multipliers.1 = 1;
    }

    if y2 - y1 < 0.0 {
        multipliers.1 = -1;
    }

    multipliers
}

pub enum InputEvents {
    Mouse(MouseEvent),
    Touch(TouchEvent),
}

pub fn get_client_values(event_type: InputEvents) -> (f64, f64) {
    match event_type {
        InputEvents::Mouse(event) => (event.client_x() as f64, event.client_y() as f64),
        InputEvents::Touch(event) => {
            let touches = event.touches();
            let touch = touches.item(touches.length() - 1).unwrap();
            (touch.client_x() as f64, touch.client_y() as f64)
        }
    }
}

pub fn get_event_location(
    nw: f64,
    nh: f64,
    rect: &DomRect,
    client_x: f64,
    client_y: f64,
) -> (f64, f64) {
    let x = client_x - rect.left();
    let y = client_y - rect.top();
    let h_ratio = nh / rect.height();
    let w_ratio = nw / rect.width();
    let converted_x = (x * w_ratio).round();
    let converted_y = (y * h_ratio).round();
    (converted_x, converted_y)
}

pub fn get_grid_dimensions(width: u32, height: u32) -> (u32, u32) {
    let mut count: u32 = 220;
    let div = (u32::max(width, height) / u32::min(width, height)) as f32;
    if div as f32 <= 1.5 {
        count = 180;
    }
    if width > height {
        (
            count as u32,
            (count as f32 / (width as f32 / height as f32) as f32) as u32,
        )
    } else if height > width {
        (
            (count as f32 / (height as f32 / width as f32) as f32) as u32,
            count as u32,
        )
    } else {
        (count, count)
    }
}

pub fn initialise_canvas(
    app: SimAppUniverseWrapper,
) -> (SimAppUniverseWrapper, web_sys::HtmlCanvasElement, u32, u32) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id(CANVAS_ID)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    resize_canvas_to_display_size(&canvas);

    let (h_cells, v_cells) = get_grid_dimensions(canvas.width(), canvas.height());

    (app, canvas, h_cells, v_cells)
}

pub fn wrld_clbk<T>(
    world: &SimAppUniverseWrapper,
    f: impl FnOnce(SimAppUniverseWrapper) -> T,
) -> T {
    let world_clone = world.clone();
    (f)(world_clone)
}

pub fn start_animation_loop(webgl_data: WebGlData, fluid: Rc<RefCell<Fluid>>) {
    let render_loop: Rc<RefCell<RenderLoop>> = Rc::new(RefCell::new(RenderLoop::new(None, None)));
    let fluid_clone = fluid.clone();
    let window = web_sys::window().unwrap();
    let closure: Closure<dyn Fn()> = {
        let render_loop = render_loop.clone();
        Closure::wrap(Box::new(move || {
            fluid_clone.borrow_mut().simulate();
            render_fluid(&webgl_data, &fluid.borrow().density);
            let mut render_loop = render_loop.borrow_mut();
            render_loop.animation_id = if let Some(ref closure) = render_loop.closure {
                Some(
                    window
                        .request_animation_frame(closure.as_ref().unchecked_ref())
                        .expect("cannot set animation frame"),
                )
            } else {
                None
            }
        }))
    };

    let window = web_sys::window().unwrap();
    let mut render_loop = render_loop.borrow_mut();
    render_loop.animation_id = Some(
        window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .expect("cannot set animation frame"),
    );
    render_loop.closure = Some(closure);
}
