use crate::GeneralCoordinateSystem3d;

#[derive(Debug, Clone, Copy)]
pub struct Torus<T = f64> {
    pub pos: GeneralCoordinateSystem3d<T>,
    pub major_radius: T,
    pub minor_radius: T,
}
