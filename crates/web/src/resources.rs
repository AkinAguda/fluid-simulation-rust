pub(crate) struct Resources {
    pub(crate) render_fn: RenderFn,
}

impl Resources {
    pub fn set_render_fn(&mut self, render_fn: RenderFn) {
        self.render_fn = render_fn
    }
}

pub type RenderFn = Box<dyn FnMut() -> ()>;
