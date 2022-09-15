mod components;
mod pages;
mod resources;
mod state;
mod utility;
mod world;

use std::{cell::RefCell, rc::Rc};

use crate::utility::constants::{DEFAULT_DIFFUSION, DEFAULT_TIME_STEP};
use crate::utility::{
    functions::initialise_canvas,
    webgl::{initialise_webgl, render_fluid},
};
use crate::world::Msg;
use crate::world::{SimAppWorldWrapper, World};
use app_world::AppWorldWrapper;
use fluid_sim::{Fluid, FluidConfig};
use pages::home::home_view::Home;
use percy_dom::prelude::*;
use percy_dom::JsCast;
use percy_dom::{render::create_render_scheduler, VirtualNode};
use resources::FluidProperySetters;
use resources::RenderFn;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{self};

#[derive(Clone)]
pub struct SimApp {
    world: SimAppWorldWrapper,
}

struct RenderLoop {
    animation_id: Option<i32>,
    pub closure: Option<Closure<dyn Fn()>>,
}

impl RenderLoop {
    fn new(animation_id: Option<i32>, closure: Option<Closure<dyn Fn()>>) -> RenderLoop {
        RenderLoop {
            animation_id,
            closure,
        }
    }
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

        let window = web_sys::window().unwrap();

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

        let fluid_ref_1 = fluid.clone();
        let fluid_ref_2 = fluid.clone();

        app2.world
            .msg(Msg::SetFluidPropertySetters(FluidProperySetters {
                time_step: Box::new(move |val: f32| {
                    fluid_ref_1.borrow_mut().set_dt(val);
                }),
                diffusion: Box::new(move |val: f32| {
                    fluid_ref_2.borrow_mut().set_diffusion(val);
                }),
            }));

        let webgl_data = initialise_webgl(&canvas, nw as f32, nh as f32);

        let render_loop: Rc<RefCell<RenderLoop>> =
            Rc::new(RefCell::new(RenderLoop::new(None, None)));

        let closure: Closure<dyn Fn()> = {
            let render_loop = render_loop.clone();
            Closure::wrap(Box::new(move || {
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

        let render = move || render_app_with_world(&app);

        let render = create_render_scheduler(pdom, render);

        app2.world.msg(Msg::SetRenderFn(render));

        WebClient {}
    }
}
