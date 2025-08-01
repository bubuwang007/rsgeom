use crate::CoordinateSystem3d;

#[derive(Debug, Clone, Copy)]
pub struct Parabola3d<T = f64> {
    pub pos: CoordinateSystem3d<T>,
    pub focal_length: T,
}
