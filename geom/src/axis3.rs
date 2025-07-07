use crate::direction3d::Direction3d;
use crate::point3d::Point3d;
use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct Axis3<T = f64> {
    pub location: Point3d<T>,
    pub direction: Direction3d<T>,
}

impl<T> std::fmt::Display for Axis3<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Axis3({}, {})", self.location, self.direction)
    }
}

impl<T> Axis3<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Axis3 {
            location: Point3d::new(),
            direction: Direction3d::new(),
        }
    }
}
