#[derive(Clone)]
pub enum FluidProperty {
    Diffusion(f32),
    TimeStep(f32),
    Density(f32),
    Velocity(f32),
}
