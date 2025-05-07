
pub trait FloatWithConst: num_traits::Float {
    fn pi() -> Self;
    fn frac_pi_2() -> Self;
}

impl FloatWithConst for f32 {
    fn pi() -> Self {
        std::f32::consts::PI
    }

    fn frac_pi_2() -> Self {
        std::f32::consts::FRAC_PI_2
    }

}

impl FloatWithConst for f64 {
    fn pi() -> Self {
        std::f64::consts::PI
    }

    fn frac_pi_2() -> Self {
        std::f64::consts::FRAC_PI_2
    }
}