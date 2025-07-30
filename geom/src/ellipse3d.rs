use crate::CoordinateSystem3d;

#[derive(Debug, Clone, Copy)]
pub struct Ellipse3d<T = f64> {
    pub position: CoordinateSystem3d<T>,
    pub major_radius: T,
    pub minor_radius: T,
}
