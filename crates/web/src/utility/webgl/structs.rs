use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlTexture};

pub struct WebGlData {
    pub nw: f32,
    pub nh: f32,
    pub context: WebGlRenderingContext,
    pub vertices: Vec<f32>,
    pub position_attribute_location: i32,
    pub density_attribute_location: i32,
    pub pos_attribute_location: i32,
    pub tex_attribute_location: i32,
    pub position_buffer: WebGlBuffer,
    pub density_buffer: WebGlBuffer,
    pub pos_buffer: WebGlBuffer,
    pub tex_coord_buffer: WebGlBuffer,
    pub texture: WebGlTexture,
    pub program_1: WebGlProgram,
    pub program_2: WebGlProgram,
}
