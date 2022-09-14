mod components;
mod pages;
mod resources;
mod state;
mod utility;
mod world;

use std::{cell::RefCell, rc::Rc};

use app_world::AppWorldWrapper;
use resources::RenderFn;
use wasm_bindgen::prelude::*;
use web_sys;

use crate::utility::{
    functions::initialise_canvas,
    webgl::{initialise_webgl, render_fluid},
};

use crate::world::Msg;

use pages::home::home_view::Home;

use percy_dom::prelude::*;

use crate::utility::constants::{DEFAULT_DIFFUSION, DEFAULT_TIME_STEP};

use crate::world::{SimAppWorldWrapper, World};
use fluid_sim::{Fluid, FluidConfig};
use percy_dom::{render::create_render_scheduler, VirtualNode};
use resources::FluidProperySetters;

#[derive(Clone)]
pub struct SimApp {
    world: SimAppWorldWrapper,
}

#[wasm_bindgen]
pub struct WebClient {}

// #[wasm_bindgen]
// pub struct App;

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

fn render_app_with_world(app: &SimApp) -> VirtualNode {
    let app_2 = app.clone();
    Home {
        world: app_2.world.clone(),
    }
    .render()
}

fn create_app(render: RenderFn) -> SimApp {
    SimApp {
        world: AppWorldWrapper::new(World::new(render)),
    }
}

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        console_error_panic_hook::set_once();
        css_mod::init!();

        let render = Box::new(|| {});
        let app: SimApp = create_app(render);
        let app2 = app.clone();
        let mut pdom = create_dom_updater();

        pdom.update(render_app_with_world(&app));

        let (app2, canvas, nw, nh) = initialise_canvas(app2);

        let fluid = Rc::new(RefCell::new(Fluid::new(FluidConfig::new(
            nw as u16,
            nh as u16,
            DEFAULT_DIFFUSION,
            DEFAULT_TIME_STEP,
        ))));

        let fluid_1 = fluid.clone();
        let fluid_2 = fluid.clone();

        app2.world
            .msg(Msg::SetFluidPropertySetters(FluidProperySetters {
                time_step: Box::new(move |val: f32| {
                    fluid_1.borrow_mut().set_dt(val);
                }),
                diffusion: Box::new(move |val: f32| {
                    fluid_2.borrow_mut().set_diffusion(val);
                }),
            }));

        let webgl_data = initialise_webgl(&canvas, nw as f32, nh as f32);

        render_fluid(&webgl_data, &fluid.borrow().density);

        let render = move || render_app_with_world(&app);

        let render = create_render_scheduler(pdom, render);

        app2.world.msg(Msg::SetRenderFn(render));

        WebClient {}
    }
}
