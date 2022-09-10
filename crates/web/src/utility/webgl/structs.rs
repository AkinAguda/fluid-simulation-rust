use js_sys::Float32Array;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlTexture};

pub struct WebGlData {
    pub nw: i32,
    pub nh: i32,
    pub context: WebGlRenderingContext,
    pub vertices: Float32Array,
    pub densities: Float32Array,
    pub position_attribute_location: i32,
    pub density_attribute_location: i32,
    pub pos_attribute_location: i32,
    pub tex_attribute_location: i32,
    pub position_buffer: WebGlBuffer,
    pub density_buffer: WebGlBuffer,
    pub pos_buffer: WebGlBuffer,
    pub tex_coord_buffer: WebGlBuffer,
    pub texture: WebGlTexture,
    // Render To Texture Program
    pub rtt_program: WebGlProgram,
    // Render To Canvas Program
    pub rtc_program: WebGlProgram,
}
