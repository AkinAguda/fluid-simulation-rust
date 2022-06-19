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
        html! {
            <div>
                Hello world
            </div>
        }
    }
}
