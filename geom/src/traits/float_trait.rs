use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub trait FloatWithConst:
    num_traits::Float + AddAssign + SubAssign + MulAssign + DivAssign + Copy + Default
{
    fn pi() -> Self;
    fn frac_pi_2() -> Self;
    fn min_positive() -> Self;
}

impl FloatWithConst for f32 {
    fn pi() -> Self {
        std::f32::consts::PI
    }

    fn frac_pi_2() -> Self {
        std::f32::consts::FRAC_PI_2
    }

    fn min_positive() -> Self {
        std::f32::MIN_POSITIVE
    }
}

impl FloatWithConst for f64 {
    fn pi() -> Self {
        std::f64::consts::PI
    }

    fn frac_pi_2() -> Self {
        std::f64::consts::FRAC_PI_2
    }

    fn min_positive() -> Self {
        std::f64::MIN_POSITIVE
    }
}
