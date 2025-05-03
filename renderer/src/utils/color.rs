
#[inline(always)]
pub fn float_to_u8(value: f32) -> u8 {
    (value * 255.0).clamp(0.0, 255.0) as u8
}

#[inline(always)]
pub fn u8_to_float(value: u8) -> f32 {
    value as f32 / 255.0
}

