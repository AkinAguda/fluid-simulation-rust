use super::constants::{FRAGMENT_SHADER_1, FRAGMENT_SHADER_2, VERTEX_SHADER_1, VERTEX_SHADER_2};
use super::structs::WebGlData;
use js_sys::Float32Array;
use percy_dom::JsCast;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext as GL, WebGlShader, WebGlTexture};

pub fn create_shader(context: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context.create_shader(shader_type).unwrap();
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let err = Err(context.get_shader_info_log(&shader).unwrap());
        context.delete_shader(Some(&shader));
        err
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

pub fn create_program(
    context: &GL,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context.create_program().unwrap();
    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);

    context.link_program(&program);

    if context
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        let err = Err(context.get_program_info_log(&program).unwrap());
        context.delete_program(Some(&program));
        err
    }
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

pub fn render_to_canvas(webgl_data: &WebGlData) {
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

    context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    context.draw_arrays(GL::TRIANGLES, 0, 6);
}

pub fn render_to_texture(webgl_data: &WebGlData) -> WebGlTexture {
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

    context.buffer_data_with_array_buffer_view(
        GL::ARRAY_BUFFER,
        &densities.borrow(),
        GL::STATIC_DRAW,
    );

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

    let texture_position_projection_location = context
        .get_uniform_location(&rtt_program, "u_texturePositionProjection")
        .unwrap();

    let canvas_projection_location = context
        .get_uniform_location(&rtc_program, "u_canvasProjection")
        .unwrap();

    let image_projection_location = context
        .get_uniform_location(&rtc_program, "u_imageProjection")
        .unwrap();

    let position_buffer = context.create_buffer().unwrap();

    let density_buffer = context.create_buffer().unwrap();

    let pos_buffer = context.create_buffer().unwrap();

    let tex_coord_buffer = context.create_buffer().unwrap();

    let texture = context.create_texture().unwrap();

    context.use_program(Some(&rtt_program));

    #[rustfmt::skip]
    let texture_position_projection_matrix = vec![
        (2.0 / nw as f32), 0.0, 0.0,
        0.0, (-2.0 / nh as f32), 0.0,
        -1.0, 1.0, 1.0 
    ];

    context.uniform_matrix3fv_with_f32_array(
        Some(&texture_position_projection_location),
        false,
        &texture_position_projection_matrix,
    );

    context.use_program(Some(&rtc_program));

    #[rustfmt::skip]
    let image_projection_matrix = vec![
        (1.0 / nw as f32), 0.0, 0.0,
        0.0, (-1.0 / nh as f32), 0.0,
        0.0, 1.0, 1.0 
    ];

    context.uniform_matrix3fv_with_f32_array(
        Some(&image_projection_location),
        false,
        &image_projection_matrix,
    );

    #[rustfmt::skip]
    let canvas_projection_matrix = vec![
        (2.0 / canvas.width() as f32), 0.0, 0.0,
        0.0, (-2.0 / canvas.height() as f32), 0.0,
        -1.0, 1.0, 1.0 
    ];

    context.uniform_matrix3fv_with_f32_array(
        Some(&canvas_projection_location),
        false,
        &canvas_projection_matrix,
    );

    // Populating vertices
    let vertices = Float32Array::new_with_length((nw * nh * 2.0) as u32);
    // TODO: Get size from fluid. This function should know nothing about the internal calculation of the fluid's size
    let densities = RefCell::new(Float32Array::new_with_length((nw * nh) as u32));
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

pub fn render_fluid(webgl_data: &WebGlData, fluid_density: &Vec<f32>) {
    /* This is unsafe because I am copying an f32 vec into Float32Array */
    unsafe {
        webgl_data
            .densities
            .replace(Float32Array::view(fluid_density));
        render_to_texture(&webgl_data);
        render_to_canvas(&webgl_data);
    }
}
