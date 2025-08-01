use crate::CoordinateSystem2d;

#[derive(Debug, Clone, Copy)]
pub struct Hyperbola2d<T = f64> {
    pub position: CoordinateSystem2d<T>,
    pub major_radius: T,
    pub minor_radius: T,
}
