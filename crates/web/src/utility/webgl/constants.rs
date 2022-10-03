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
uniform vec2 u_canvasResolution;
uniform vec2 u_imageResolution;
varying vec2 v_texCoord;

{}

vec2 convertToTextureClipSpace(vec2 position, vec2 resolution) {{
  vec2 zeroToOne = position / resolution;
  
  vec2 clipSpace = vec2(zeroToOne.x, 1.0 - zeroToOne.y);
  
  return clipSpace;
}}

void main() {{
  gl_Position = vec4(convertToClipSpace(a_pos, u_canvasResolution), 0, 1);
  v_texCoord = convertToTextureClipSpace(a_texCoord, u_imageResolution);
}}
"#,
    CONVERT_TO_CLIP_SPACE
);

pub const FRAGMENT_SHADER_1: &str = r#"
precision mediump float;
varying float v_density;

float interpolateToZeroToOne(float start, float end, float value) {{
  // return (value - start) / (end - start);
  return value;
}}

void main() {
  float range_density = interpolateToZeroToOne(0.0, 20.0, v_density);
  gl_FragColor = vec4(sin(range_density), sin(range_density) * cos(range_density) * sin(range_density), sin(range_density / 2.0), 1.0);
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
