use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub fn create_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context.create_shader(shader_type).unwrap();
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
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

pub fn create_program(
    context: &WebGlRenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context.create_program().unwrap();
    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);

    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
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
