mod components;
mod pages;
mod resources;
mod simulation;
mod state;
mod utility;
mod world;

use std::rc::Rc;

use app_world::AppWorldWrapper;
use resources::RenderFn;
use wasm_bindgen::prelude::*;
use web_sys;

use crate::world::Msg;

use pages::home::home_view::Home;

use percy_dom::prelude::*;

use crate::world::{SimAppWorldWrapper, World};
use percy_dom::{render::create_render_scheduler, VElement, VirtualNode};

pub struct SimApp {
    world: SimAppWorldWrapper,
}

pub struct WebClient {
    app: SimApp,
    pdom: PercyDom,
}

#[wasm_bindgen]
pub struct App;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
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

fn render_app_with_world(app: &'static SimApp) -> VirtualNode {
    // render_home(app.world)
    Home {
        msg_say_hello: Rc::new(|text: String| {
            app.world.msg(Msg::SetBtnTxt(text));
        }),
        button_text: app.world.read().state.btn_txt,
    }
    .render()
}

fn create_app(render: RenderFn) -> SimApp {
    SimApp {
        world: AppWorldWrapper::new(World::new(render)),
    }
}

impl WebClient {
    pub fn new() -> WebClient {
        let render = Box::new(|| {});
        let app: SimApp = create_app(render);
        let mut pdom = create_dom_updater();
        pdom.update(render_app_with_world(&app));
        WebClient { app, pdom }
    }

    pub fn start(mut self) {
        let render = move || render_app_with_world(&self.app);
        let render = create_render_scheduler(self.pdom, render);
        // self.app = create_app(render);
    }
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> App {
        css_mod::init!();

        let client = WebClient::new();
        client.start();

        App
    }
}
