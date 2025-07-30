use crate::Axis3d;
use crate::Direction3d;
use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct GeneralCoordinateSystem3d<T = f64> {
    pub axis: Axis3d<T>,
    pub vydir: Direction3d<T>,
    pub vxdir: Direction3d<T>,
}

impl<T> std::fmt::Display for GeneralCoordinateSystem3d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GeneralCoordinateSystem3d(axis: {}, vydir: {}, vxdir: {})",
            self.axis, self.vydir, self.vxdir
        )
    }
}
