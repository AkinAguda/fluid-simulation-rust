use const_format::formatcp;

const CONVERT_TO_CLIP_SPACE: &str = r#"
vec2 convertToClipSpace(vec2 position, vec2 resolution) {
    vec2 zeroToOne = position / resolution;
    
    vec2 zeroToTwo = zeroToOne * 2.0;
    
    vec2 clipSpace = vec2(zeroToTwo.x - 1.0, 1.0 - zeroToTwo.y);
    
    return clipSpace;
  }
"#;

pub const VERTEX_SHADER_1: &str = formatcp!(
    r#"
  attribute vec2 a_position;
  attribute float a_density;
  uniform vec2 u_resolution;
  varying float v_density;
  {}
  void main() {{
      gl_Position = vec4(convertToClipSpace(a_position, u_resolution), 0, 1);
      gl_PointSize = 1.0;
      v_density = a_density;
  }}
  "#,
    CONVERT_TO_CLIP_SPACE
);
pub const VERTEX_SHADER_2: &str = formatcp!(
    r#"
attribute vec2 a_pos;
attribute vec2 a_texCoord;
uniform vec2 u_imageResolution;
uniform mat3 u_canvasProjection;
varying vec2 v_texCoord;

{}

vec2 convertToTextureClipSpace(vec2 position, vec2 resolution) {{
  vec2 zeroToOne = position / resolution;
  
  vec2 clipSpace = vec2(zeroToOne.x, 1.0 - zeroToOne.y);
  
  return clipSpace;
}}

void main() {{
  vec2 position = (u_canvasProjection * vec3(a_pos, 1)).xy;
  gl_Position = vec4(position, 0, 1);
  v_texCoord = convertToTextureClipSpace(a_texCoord, u_imageResolution);
}}
"#,
    CONVERT_TO_CLIP_SPACE
);

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
