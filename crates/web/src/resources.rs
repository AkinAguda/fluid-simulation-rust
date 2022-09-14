pub struct FluidProperySetters {
    pub time_step: Box<dyn Fn(f32) -> ()>,
    pub diffusion: Box<dyn Fn(f32) -> ()>,
}

pub(crate) struct Resources {
    pub(crate) render_fn: RenderFn,
    pub fluid_propery_setters: FluidProperySetters,
}

impl Resources {
    pub fn set_render_fn(&mut self, render_fn: RenderFn) {
        self.render_fn = render_fn
    }

    pub fn set_fluid_propery_setters(&mut self, fluid_propery_setters: FluidProperySetters) {
        self.fluid_propery_setters = fluid_propery_setters;
    }
}

pub type RenderFn = Box<dyn FnMut() -> ()>;
