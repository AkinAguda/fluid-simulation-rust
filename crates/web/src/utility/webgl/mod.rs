use fluid_sim::Fluid;
use js_sys::Float32Array;
use percy_dom::JsCast;
use web_sys::{WebGlRenderingContext as GL, WebGlTexture};

use self::functions::{create_program, create_shader};

mod constants;
mod functions;
pub mod structs;

use constants::{FRAGMENT_SHADER_1, FRAGMENT_SHADER_2, VERTEX_SHADER_1, VERTEX_SHADER_2};
use structs::WebGlData;

pub fn initialise_webgl(canvas: &web_sys::HtmlCanvasElement, nw: f32, nh: f32) -> WebGlData {
    let context = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<GL>()
        .unwrap();

    let vertex_shader_1 = create_shader(&context, GL::VERTEX_SHADER, VERTEX_SHADER_1).unwrap();

    let fragment_shader_1 =
        create_shader(&context, GL::FRAGMENT_SHADER, FRAGMENT_SHADER_1).unwrap();

    let rtt_program = create_program(&context, &vertex_shader_1, &fragment_shader_1).unwrap();

    let vertex_shader_2 = create_shader(&context, GL::VERTEX_SHADER, VERTEX_SHADER_2).unwrap();

    let fragment_shader_2 =
        create_shader(&context, GL::FRAGMENT_SHADER, FRAGMENT_SHADER_2).unwrap();

    let rtc_program = create_program(&context, &vertex_shader_2, &fragment_shader_2).unwrap();

    let position_attribute_location = context.get_attrib_location(&rtt_program, "a_position");

    let density_attribute_location = context.get_attrib_location(&rtt_program, "a_density");

    let pos_attribute_location = context.get_attrib_location(&rtc_program, "a_pos");

    let tex_attribute_location = context.get_attrib_location(&rtc_program, "a_texCoord");

    let resolution_uniform_location = context
        .get_uniform_location(&rtt_program, "u_resolution")
        .unwrap();

    let canvas_resolution = context
        .get_uniform_location(&rtc_program, "u_canvasResolution")
        .unwrap();

    let image_resolution = context
        .get_uniform_location(&rtc_program, "u_imageResolution")
        .unwrap();

    let position_buffer = context.create_buffer().unwrap();

    let density_buffer = context.create_buffer().unwrap();

    let pos_buffer = context.create_buffer().unwrap();

    let tex_coord_buffer = context.create_buffer().unwrap();

    let texture = context.create_texture().unwrap();

    context.use_program(Some(&rtt_program));

    context.uniform2f(Some(&resolution_uniform_location), nw, nh);

    context.use_program(Some(&rtc_program));

    context.uniform2f(
        Some(&canvas_resolution),
        canvas.width() as f32,
        canvas.height() as f32,
    );

    context.uniform2f(Some(&image_resolution), nw, nh);

    // Populating vertices
    let mut vertices = Float32Array::new_with_length((nw * nh * 2.0) as u32);
    let densities = Float32Array::new_with_length((nw * nh) as u32);
    // let density_per_square: Vec<f32> = vec![0.0; (nw * nh * 2.0) as usize];
    let mut point_index: u32 = 0;
    const HALF_SQUARE: f32 = 0.5;

    for i in 1..=nh as u32 {
        for j in 1..=nw as u32 {
            vertices.set_index(point_index, j as f32 - HALF_SQUARE);
            vertices.set_index(point_index + 1, i as f32 - HALF_SQUARE);
            point_index += 2;
        }
    }

    WebGlData {
        nw: nw as i32,
        nh: nh as i32,
        context,
        vertices,
        densities,
        position_attribute_location,
        density_attribute_location,
        pos_attribute_location,
        tex_attribute_location,
        position_buffer,
        density_buffer,
        pos_buffer,
        tex_coord_buffer,
        texture,
        rtt_program,
        rtc_program,
    }
}

fn render_to_texture(webgl_data: &WebGlData) -> WebGlTexture {
    let WebGlData {
        context,
        nw,
        nh,
        rtt_program,
        position_buffer,
        vertices,
        density_buffer,
        densities,
        position_attribute_location,
        density_attribute_location,
        ..
    } = webgl_data;

    let nw = *nw;
    let nh = *nh;
    let position_attribute_location = *position_attribute_location;
    let density_attribute_location = *density_attribute_location;

    let target_texture = context.create_texture().unwrap();
    context.bind_texture(GL::TEXTURE_2D, Some(&target_texture));

    context.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    context.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
    context.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
    context.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);

    context
        .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            nw,
            nh,
            0,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            None,
        )
        .unwrap();

    let fb = context.create_framebuffer().unwrap();
    context.bind_framebuffer(GL::FRAMEBUFFER, Some(&fb));

    let attachment_point = GL::COLOR_ATTACHMENT0;
    context.framebuffer_texture_2d(
        GL::FRAMEBUFFER,
        attachment_point,
        GL::TEXTURE_2D,
        Some(&target_texture),
        0,
    );

    // Render Code
    context.use_program(Some(&rtt_program));

    context.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));

    context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices, GL::STATIC_DRAW);

    context.bind_buffer(GL::ARRAY_BUFFER, Some(&density_buffer));

    context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &densities, GL::STATIC_DRAW);

    context.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));

    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        2,
        GL::FLOAT,
        false,
        0,
        0,
    );

    context.bind_buffer(GL::ARRAY_BUFFER, Some(&density_buffer));

    context.enable_vertex_attrib_array(density_attribute_location as u32);

    context.vertex_attrib_pointer_with_i32(
        density_attribute_location as u32,
        1,
        GL::FLOAT,
        true,
        0,
        0,
    );

    context.viewport(0, 0, nw, nh);
    context.clear_color(0.0, 0.0, 0.0, 0.0);
    context.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    context.draw_arrays(GL::POINTS, 0, nw * nh);

    target_texture
}

fn render_to_canvas(webgl_data: &WebGlData) {
    let WebGlData {
        context,
        nw,
        nh,
        rtc_program,
        pos_buffer,
        tex_coord_buffer,
        pos_attribute_location,
        tex_attribute_location,
        ..
    } = webgl_data;

    let canvas = context
        .canvas()
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let nw = *nw;
    let nh = *nh;

    let pos_attribute_location = *pos_attribute_location as u32;
    let tex_attribute_location = *tex_attribute_location as u32;

    context.use_program(Some(&rtc_program));

    context.bind_buffer(GL::ARRAY_BUFFER, Some(&pos_buffer));

    set_rectange(
        &context,
        0.0,
        0.0,
        canvas.width() as f32,
        canvas.height() as f32,
    );

    context.bind_buffer(GL::ARRAY_BUFFER, Some(&tex_coord_buffer));

    set_rectange(&context, 0.0, 0.0, nw as f32, nh as f32);

    context.bind_buffer(GL::ARRAY_BUFFER, Some(&pos_buffer));

    context.enable_vertex_attrib_array(pos_attribute_location);

    context.vertex_attrib_pointer_with_i32(pos_attribute_location, 2, GL::FLOAT, false, 0, 0);

    context.bind_buffer(GL::ARRAY_BUFFER, Some(&tex_coord_buffer));

    context.enable_vertex_attrib_array(tex_attribute_location);

    context.vertex_attrib_pointer_with_i32(tex_attribute_location, 2, GL::FLOAT, false, 0, 0);

    context.bind_framebuffer(GL::FRAMEBUFFER, None);

    context.draw_arrays(GL::TRIANGLES, 0, 6);
}

fn set_rectange(context: &GL, x: f32, y: f32, width: f32, height: f32) {
    let x1 = x;
    let x2 = x + width;
    let y1 = y;
    let y2 = y + height;

    let arr = Float32Array::new_with_length(12);

    arr.copy_from(&[x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2]);

    context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &arr, GL::STATIC_DRAW);
}

pub fn render_fluid(webgl_data: &WebGlData, fluid_density: &Vec<f32>) {
    webgl_data.densities.copy_from(fluid_density);
    render_to_texture(&webgl_data);
    render_to_canvas(&webgl_data);
}
