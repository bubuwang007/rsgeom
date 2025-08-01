use crate::CoordinateSystem2d;

#[derive(Debug, Clone, Copy)]
pub struct Parabola2d<T = f64> {
    pub pos: CoordinateSystem2d<T>,
    pub focal_length: T,
}