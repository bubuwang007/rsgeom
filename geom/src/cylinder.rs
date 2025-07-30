use crate::GeneralCoordinateSystem3d;

#[derive(Debug, Clone, Copy)]
pub struct Cylinder<T = f64> {
    pub position: GeneralCoordinateSystem3d<T>,
    pub radius: T,
}
