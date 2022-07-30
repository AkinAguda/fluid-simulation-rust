use std::rc::Rc;

use crate::world::{SimAppWorldWrapper, Msg};

pub struct ConfigData {
    pub time_step: f32,
    pub diffusion: f32,
    pub density: f32,
    pub velocity: f32,
}
