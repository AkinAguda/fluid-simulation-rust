use percy_dom::JsCast;
use web_sys::{WebGlRenderingContext, WebGlTexture};

use self::functions::{create_program, create_shader};

mod constants;
mod functions;
mod structs;

use constants::{FRAGMENT_SHADER_1, FRAGMENT_SHADER_2, VERTEX_SHADER_1, VERTEX_SHADER_2};
use structs::WebGlData;

pub fn initialise_webgl(canvas: &web_sys::HtmlCanvasElement, nw: f32, nh: f32) -> WebGlData {
    let context = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    let vertex_shader_1 = create_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        VERTEX_SHADER_1,
    )
    .unwrap();

    let fragment_shader_1 = create_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        FRAGMENT_SHADER_1,
    )
    .unwrap();

    let program_1 = create_program(&context, &vertex_shader_1, &fragment_shader_1).unwrap();

    let vertex_shader_2 = create_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        VERTEX_SHADER_2,
    )
    .unwrap();

    let fragment_shader_2 = create_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        FRAGMENT_SHADER_2,
    )
    .unwrap();

    let program_2 = create_program(&context, &vertex_shader_2, &fragment_shader_2).unwrap();

    let position_attribute_location = context.get_attrib_location(&program_1, "a_position");

    let density_attribute_location = context.get_attrib_location(&program_1, "a_density");

    let pos_attribute_location = context.get_attrib_location(&program_2, "a_pos");

    let tex_attribute_location = context.get_attrib_location(&program_2, "a_texCoord");

    let resolution_uniform_location = context
        .get_uniform_location(&program_1, "u_resolution")
        .unwrap();

    let canvas_resolution = context
        .get_uniform_location(&program_2, "u_canvasResolution")
        .unwrap();

    let image_resolution = context
        .get_uniform_location(&program_2, "u_imageResolution")
        .unwrap();

    let position_buffer = context.create_buffer().unwrap();

    let density_buffer = context.create_buffer().unwrap();

    let pos_buffer = context.create_buffer().unwrap();

    let tex_coord_buffer = context.create_buffer().unwrap();

    let texture = context.create_texture().unwrap();

    context.use_program(Some(&program_1));

    context.uniform2f(Some(&resolution_uniform_location), nw, nh);

    context.use_program(Some(&program_2));

    context.uniform2f(
        Some(&canvas_resolution),
        canvas.width() as f32,
        canvas.height() as f32,
    );

    context.uniform2f(Some(&image_resolution), nw, nh);

    // Populating vertices
    let mut vertices: Vec<f32> = vec![0.0; (nw * nh * 2.0) as usize];
    // let density_per_square: Vec<f32> = vec![0.0; (nw * nh * 2.0) as usize];
    let mut point_index = 0.5;
    const HALF_SQUARE: f32 = 0.5;

    for i in 1..=nh as u32 {
        for j in 1..=nw as u32 {
            vertices[point_index as usize] = j as f32 - HALF_SQUARE;
            vertices[(point_index + 1.0) as usize] = i as f32 - HALF_SQUARE;
            point_index += 2.0;
        }
    }

    WebGlData {
        nw,
        nh,
        context,
        vertices,
        position_attribute_location,
        density_attribute_location,
        pos_attribute_location,
        tex_attribute_location,
        position_buffer,
        density_buffer,
        pos_buffer,
        tex_coord_buffer,
        texture,
        program_1,
        program_2,
    }
}

fn render_to_texture(webglData: WebGlData) -> WebGlTexture {
    let WebGlData {
        context, nw, nh, ..
    } = webglData;

    let target_texture = context.create_texture().unwrap();
    context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&target_texture));

    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_S,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_T,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MAG_FILTER,
        WebGlRenderingContext::LINEAR as i32,
    );

    context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
        WebGlRenderingContext::TEXTURE_2D,
        0,
        WebGlRenderingContext::RGBA as i32,
        nw as i32,
        nh as i32,
        0,
        WebGlRenderingContext::RGBA,
        WebGlRenderingContext::UNSIGNED_BYTE,
        None,
    );

    // Finish render to texture function
}
