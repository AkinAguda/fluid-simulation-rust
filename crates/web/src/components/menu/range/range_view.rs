use std::rc::Rc;

use percy_dom::*;
use web_sys::HtmlInputElement;

pub struct Range<'a> {
    pub key: &'a str,
    pub title: &'a str,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub oninput: Rc<dyn Fn(f32) -> ()>,
}

fn get_progress_bar_width(value: f32, max: f32) -> f32 {
    (value / max) * 100.0
}

pub fn get_input_value(event: web_sys::Event, max: f32, min: f32) -> f32 {
    let val: f32 = event
        .current_target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse()
        .unwrap();
    if min >= val {
        return min;
    } else if max <= val {
        return max;
    }
    val
}

impl<'a> View for Range<'a> {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("range.css");
        let on_range_input = self.oninput.clone();
        let on_text_input = self.oninput.clone();
        let min = self.min;
        let max = self.max;

        html! {
            <li>
                <div>
                    <div class=css["title"]>{self.title}</div>

                    <div class=css["controllers"]>
                        <div class=css["meter"]>
                            <input
                                type="range"
                                oninput=move |event: web_sys::Event| {
                                    (on_range_input)(get_input_value(event, max, min));
                                }
                                value=self.value
                                step=self.step
                                max=self.max
                                min=self.min
                                class=css["range"]
                            />
                            <div class=css["progress"] style=format!("width: {:?}%", get_progress_bar_width(self.value, self.max))></div>
                            <div class=css["track"]></div>
                        </div>

                        <input
                            type="number"
                            onchange=move |event: web_sys::Event| {
                                (on_text_input)(get_input_value(event, max, min));
                            }
                            step=self.step
                            class=css["value-box"]
                            value=self.value
                        />
                    </div>
                </div>
            </li>
        }
    }
}
