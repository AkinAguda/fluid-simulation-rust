pub struct FluidProperySetters {
    pub time_step: Box<dyn Fn(f32) -> ()>,
    pub diffusion: Box<dyn Fn(f32) -> ()>,
}

pub(crate) struct Resources {
    pub fluid_propery_setters: FluidProperySetters,
}

impl Resources {
    pub fn set_fluid_propery_setters(&mut self, fluid_propery_setters: FluidProperySetters) {
        self.fluid_propery_setters = fluid_propery_setters;
    }
}
