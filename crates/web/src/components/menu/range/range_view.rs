use percy_dom::*;

pub struct Range<'a> {
    pub key: &'a str,
    pub title: &'a str,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

impl<'a> View for Range<'a> {
    fn render(&self) -> VirtualNode {
        let css = css_mod::get!("range.css");
        html! {
            <li>
                <div>
                    <div class=css["title"]>{self.title}</div>
                    
                    <div class=css["controllers"]>
                        <div class=css["meter"]>
                            <input type="range" step=self.step max=self.max min=self.min class=css["range"]/>
                            <div class=css["progress"]></div>
                            <div class=css["track"]></div>
                        </div>

                        <input type="number" step=self.step class=css["value-box"] value=self.value />
                    </div>
                </div>
            </li>
        }
    }
}
