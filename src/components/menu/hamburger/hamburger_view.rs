use crate::{
    log,
    world::{Msg, SimAppWorldWrapper, World},
};
use app_world::AppWorldWrapper;
use percy_dom::*;

// pub struct Hamburger {
//     pub world: AppWorldWrapper<World>,
// }

// fn toggle_hamburger(world: SimAppWorldWrapper) {
//     world.msg(Msg::ToggleConfig);
// }

// impl View for Hamburger {
//     fn render(&self) -> VirtualNode {
//         let css = css_mod::get!("hamburger.css");
//         html! {
//             <button
//                 class=css["config-trigger"]
//                 on_click=move|| {
//                 self.world.msg(Msg::ToggleConfig);
//             }>
//                 <div />
//                 <div />
//                 <div />
//             </button>
//         }
//     }
// }

pub fn render_hamburger(wrap: SimAppWorldWrapper) -> VirtualNode {
    let css = css_mod::get!("hamburger.css");
    html! {
        <button
            class=css["config-trigger"]
            onclick= move|| {
                log("LOGGED IN HAMBURGER CALLBACK");
                wrap.msg(Msg::ToggleConfig);
            }
        >
            <div />
            <div />
            <div />
        </button>
    }
}
