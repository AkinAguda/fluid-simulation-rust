mod constants;
mod utils;

use constants::GAUSS_SEIDEL_ITERATIONS;
use std::cmp;
use utils::{lerp, pure_ix_fn, set_panic_hook, BoundaryType, PropertyType};

pub struct FluidConfig {
    pub nw: u16,
    pub nh: u16,
    pub diffusion: f32,
    pub dt: f32,
    pub size: u16,
}

impl FluidConfig {
    pub fn new(nw: u16, nh: u16, diffusion: f32, dt: f32) -> FluidConfig {
        let size = (nw + 2) * (nh + 2);
        FluidConfig {
            nw,
            nh,
            diffusion,
            size,
            dt,
        }
    }

    pub fn update_Size(&mut self, nw: u16, nh: u16) {
        self.nw = nw;
        self.nh = nh;
        self.size = (nw + 2) * (nh + 2);
    }
}

pub struct Fluid {
    pub config: FluidConfig,
    empty_property: PropertyType,
    velocity_x: PropertyType,
    velocity_y: PropertyType,
    initial_velocity_x: PropertyType,
    initial_velocity_y: PropertyType,
    velocity_x_source: PropertyType,
    velocity_y_source: PropertyType,
    pub density: PropertyType,
    initial_density: PropertyType,
    density_source: PropertyType,
    poisson_values: PropertyType,
    divergence_values: PropertyType,
}

impl Fluid {
    pub fn new(config: FluidConfig) -> Fluid {
        set_panic_hook();
        let vector_size = config.size.into();
        Fluid {
            config,
            empty_property: vec![0.0; vector_size],
            velocity_x: vec![0.0; vector_size],
            velocity_y: vec![0.0; vector_size],
            initial_velocity_x: vec![0.0; vector_size],
            initial_velocity_y: vec![0.0; vector_size],
            velocity_x_source: vec![0.0; vector_size],
            velocity_y_source: vec![0.0; vector_size],
            density: vec![0.0; vector_size],
            initial_density: vec![0.0; vector_size],
            density_source: vec![0.0; vector_size],
            poisson_values: vec![0.0; vector_size],
            divergence_values: vec![0.0; vector_size],
        }
    }

    fn density_step(&mut self) {
        add_source!(
            self.initial_density,
            self.density_source,
            self.config.size as usize,
            self.config.dt
        );

        diffuse!(
            self.config.nw,
            self.config.nh,
            BoundaryType::NONE,
            self.density,
            self.initial_density,
            self.config.diffusion,
            self.config.dt
        );

        std::mem::swap(&mut self.density, &mut self.initial_density);

        advect!(
            self.config.nw,
            self.config.nh,
            BoundaryType::NONE,
            self.density,
            self.initial_density,
            self.velocity_x,
            self.velocity_y,
            self.config.dt
        );

        std::mem::swap(&mut self.density, &mut self.initial_density);
    }

    fn velocity_step(&mut self) {
        add_source!(
            self.initial_velocity_x,
            self.velocity_x_source,
            self.config.size as usize,
            self.config.dt
        );

        add_source!(
            self.initial_velocity_y,
            self.velocity_y_source,
            self.config.size as usize,
            self.config.dt
        );

        diffuse!(
            self.config.nw,
            self.config.nh,
            BoundaryType::VERTICAL,
            self.velocity_x,
            self.initial_velocity_x,
            self.config.diffusion,
            self.config.dt
        );

        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);

        diffuse!(
            self.config.nw,
            self.config.nh,
            BoundaryType::HORIZONTAL,
            self.velocity_y,
            self.initial_velocity_y,
            self.config.diffusion,
            self.config.dt
        );

        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);

        project!(
            self.config.nw,
            self.config.nh,
            self.velocity_x,
            self.velocity_y,
            self.poisson_values,
            self.divergence_values
        );

        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);
        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);

        advect!(
            self.config.nw,
            self.config.nh,
            BoundaryType::VERTICAL,
            self.velocity_x,
            self.initial_velocity_x,
            self.initial_velocity_x,
            self.initial_velocity_y,
            self.config.dt
        );

        advect!(
            self.config.nw,
            self.config.nh,
            BoundaryType::HORIZONTAL,
            self.velocity_y,
            self.initial_velocity_y,
            self.initial_velocity_x,
            self.initial_velocity_y,
            self.config.dt
        );
        project!(
            self.config.nw,
            self.config.nh,
            self.velocity_x,
            self.velocity_y,
            self.poisson_values,
            self.divergence_values
        );

        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);
        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);
    }

    // All public methods

    pub fn clear(&mut self) {
        self.velocity_x = self.empty_property.clone();
        self.velocity_y = self.empty_property.clone();
        self.initial_velocity_x = self.empty_property.clone();
        self.initial_velocity_y = self.empty_property.clone();
        self.velocity_x_source = self.empty_property.clone();
        self.velocity_y_source = self.empty_property.clone();
        self.density = self.empty_property.clone();
        self.initial_density = self.empty_property.clone();
        self.density_source = self.empty_property.clone();
        self.poisson_values = self.empty_property.clone();
        self.divergence_values = self.empty_property.clone();
    }

    pub fn add_density(&mut self, index: usize, value: f32) {
        self.density_source[index] = value;
    }

    pub fn add_velocity(&mut self, index: usize, value_x: f32, value_y: f32) {
        self.velocity_x_source[index] = value_x;
        self.velocity_y_source[index] = value_y;
    }

    pub fn simulate(&mut self) {
        self.velocity_step();
        self.density_step();
    }

    pub fn get_density_at_index(&self, index: usize) -> f32 {
        self.density[index]
    }

    pub fn ix(&self, x: u16, y: u16) -> u16 {
        pure_ix_fn(x, y, self.config.nw, self.config.nh) as u16
    }

    pub fn get_density_expensive(&self) -> PropertyType {
        self.density.clone()
    }

    pub fn get_velocity_x_expensive(&self) -> PropertyType {
        self.velocity_x.clone()
    }

    pub fn get_velocity_y_expensive(&self) -> PropertyType {
        self.velocity_y.clone()
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.config.dt = dt
    }

    pub fn set_diffusion(&mut self, diffusion: f32) {
        self.config.diffusion = diffusion
    }
}
