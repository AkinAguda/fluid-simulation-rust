mod components;
mod pages;
mod resources;
mod state;
mod utility;
mod world;

use crate::utility::constants::{DEFAULT_DIFFUSION, DEFAULT_TIME_STEP};
use crate::utility::structs::{ConfigData, MouseState};
use crate::utility::{
    functions::{get_multipliers, initialise_canvas},
    webgl::initialise_webgl,
};
use crate::world::Msg;
use crate::world::{SimAppWorldWrapper, World};
use app_world::AppWorldWrapper;
use fluid_sim::{Fluid, FluidConfig};
use pages::home::home_view::Home;
use percy_dom::prelude::*;
use percy_dom::{render::create_render_scheduler, VirtualNode};
use resources::FluidProperySetters;
use resources::RenderFn;
use std::{cell::RefCell, rc::Rc};
use utility::functions::start_animation_loop;
use wasm_bindgen::prelude::*;
use web_sys;

#[derive(Clone)]
pub struct SimApp {
    world: SimAppWorldWrapper,
}

#[wasm_bindgen]
pub struct WebClient {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

fn create_dom_updater() -> PercyDom {
    let start_view = html! { <div> Hello World </div> };
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let pdom = PercyDom::new_append_to_mount(start_view, &body);
    pdom
}

fn render_app_with_world(app: &SimApp, mouse_state: MouseStateRef) -> VirtualNode {
    let app_2 = app.clone();
    Home {
        world: app_2.world.clone(),
        mouse_state,
    }
    .render()
}

fn create_app(render: RenderFn) -> SimApp {
    SimApp {
        world: AppWorldWrapper::new(World::new(render)),
    }
}

pub type MouseStateRef = Rc<RefCell<MouseState>>;

pub type AddPropertiesFn = Rc<dyn Fn((i32, i32), (i32, i32)) -> ()>;

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        console_error_panic_hook::set_once();
        css_mod::init!();

        let render = Box::new(|| {});
        let app: SimApp = create_app(render);
        let app2 = app.clone();
        let app_ref_3 = app.clone();
        let mut pdom = create_dom_updater();

        let mouse_state: MouseStateRef = Rc::new(RefCell::new(MouseState::default()));

        pdom.update(render_app_with_world(&app, mouse_state.clone()));

        let (app2, canvas, nw, nh) = initialise_canvas(app2);

        let fluid = Rc::new(RefCell::new(Fluid::new(FluidConfig::new(
            nw as u16,
            nh as u16,
            DEFAULT_DIFFUSION,
            DEFAULT_TIME_STEP,
        ))));

        let fluid_ref_1 = fluid.clone();
        let fluid_ref_2 = fluid.clone();
        let fluid_ref_3 = fluid.clone();

        app2.world
            .msg(Msg::SetFluidPropertySetters(FluidProperySetters {
                time_step: Box::new(move |val: f32| {
                    fluid_ref_1.borrow_mut().set_dt(val);
                }),
                diffusion: Box::new(move |val: f32| {
                    fluid_ref_2.borrow_mut().set_diffusion(val);
                }),
            }));

        let mouse_state_ref = mouse_state.clone();
        let canvas_ref = &canvas;
        let add_properties_from_mouse_loc = move |coords: (i32, i32), client_values: (i32, i32)| {
            let ConfigData {
                velocity, density, ..
            } = app_ref_3.world.read().state.config_data;

            let rect = canvas_ref.get_bounding_client_rect();

            let event_x = client_values.0 as f64 - rect.left();
            let event_y = client_values.1 as f64 - rect.top();

            let prev_pos = mouse_state_ref.borrow().pos;

            let (multi_x, multi_y) = get_multipliers(prev_pos.0, prev_pos.1, event_x, event_y);

            fluid_ref_3.borrow_mut().add_velocity(
                fluid_ref_3.borrow().ix(coords.0 as u16, coords.1 as u16) as usize,
                velocity * multi_x as f32,
                velocity * multi_y as f32,
            );

            fluid_ref_3.borrow_mut().add_density(
                fluid_ref_3.borrow().ix(coords.0 as u16, coords.1 as u16) as usize,
                density,
            );

            mouse_state_ref.borrow_mut().pos = (event_x, event_y);
        };

        let webgl_data = initialise_webgl(&canvas, nw as f32, nh as f32);

        start_animation_loop(webgl_data, fluid.clone());

        let render = move || render_app_with_world(&app, mouse_state.clone());

        let render = create_render_scheduler(pdom, render);

        app2.world.msg(Msg::SetRenderFn(render));

        WebClient {}
    }
}
