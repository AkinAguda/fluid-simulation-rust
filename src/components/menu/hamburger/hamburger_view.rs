use percy_dom::*;

pub struct Hamburger {}

impl View for Hamburger {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("hamburger.css");

        html! {
            <button class=css["config-trigger"]>
                <div />
                <div />
                <div />
            </button>
        }
    }
}
