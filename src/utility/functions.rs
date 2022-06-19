pub fn round_f32(number: f32, precision: f32) -> f32 {
    ((number + f32::EPSILON) * precision) / precision
}
