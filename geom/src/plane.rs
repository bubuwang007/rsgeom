use crate::GeneralCoordinateSystem3d;

#[derive(Debug, Clone, Copy)]
pub struct Plane<T = f64> {
    pub pos: GeneralCoordinateSystem3d<T>,
}
