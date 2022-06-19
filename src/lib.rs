// contents of src/lib.rs

mod components;
mod pages;
mod simulation;
mod utility;

use pages::home::Home;
use wasm_bindgen::prelude::*;
use web_sys;

use percy_dom::prelude::*;

#[wasm_bindgen]
struct App {
    pdom: PercyDom,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> App {
        let start_view = html! { <div> Hello </div> };

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let mut pdom = PercyDom::new_append_to_mount(start_view, &body);

        let end_view = html! {
          <Home />
        };

        pdom.update(end_view);

        App { pdom }
    }
}
