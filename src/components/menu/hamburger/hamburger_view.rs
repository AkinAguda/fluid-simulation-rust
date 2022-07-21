use percy_dom::*;
use std::rc::Rc;

pub struct Hamburger {
    pub toggle_config: Rc<dyn Fn() -> ()>,
}

impl View for Hamburger {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("hamburger.css");
        let toggle_config = self.toggle_config.clone();
        html! {
            <button
                class=css["config-trigger"]
                onclick=move || {
                    (toggle_config)()
                }
            >
                <div />
                <div />
                <div />
            </button>
        }
    }
}
