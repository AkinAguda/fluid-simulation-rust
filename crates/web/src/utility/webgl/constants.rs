pub const VERTEX_SHADER_1: &str = r#"
  attribute vec2 a_position;
  attribute float a_density;
  uniform mat3 u_texturePositionProjection;
  varying float v_density;

  void main() {
    vec2 position = (u_texturePositionProjection * vec3(a_position, 1)).xy;
    gl_Position = vec4(position, 0, 1);
    gl_PointSize = 1.0;
    v_density = a_density;
  }
  "#;

pub const VERTEX_SHADER_2: &str = r#"
  attribute vec2 a_pos;
  attribute vec2 a_texCoord;
  uniform mat3 u_imageProjection;
  uniform mat3 u_canvasProjection;
  varying vec2 v_texCoord;

  void main() {
    vec2 position = (u_canvasProjection * vec3(a_pos, 1)).xy;
    gl_Position = vec4(position, 0, 1);
    v_texCoord = (u_imageProjection * vec3(a_texCoord, 1)).xy;
  }
"#;

pub const FRAGMENT_SHADER_1: &str = r#"
  precision mediump float;
  varying float v_density;

  void main() {
    gl_FragColor = vec4(v_density * v_density, v_density * 0.0, v_density * 1.0, 1.0);
  }
"#;

pub const FRAGMENT_SHADER_2: &str = r#"
  precision mediump float;
  uniform sampler2D u_texture;
  varying vec2 v_texCoord;
  void main() {
    gl_FragColor = texture2D(u_texture, v_texCoord).rgba;
  }
"#;
