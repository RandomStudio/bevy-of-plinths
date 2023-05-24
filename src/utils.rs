pub fn map_range(val: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (val - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}
