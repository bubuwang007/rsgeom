use crate::GeneralCoordinateSystem3d;

#[derive(Debug, Clone, Copy)]
pub struct Sphere<T = f64> {
    pub pos: GeneralCoordinateSystem3d<T>,
    pub radius: T,
}
