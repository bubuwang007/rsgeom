use crate::CoordinateSystem3d;

#[derive(Debug, Clone, Copy)]
pub struct Circle3d<T = f64> {
    pub position: CoordinateSystem3d<T>,
    pub radius: T,
}

impl<T> std::fmt::Display for Circle3d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Circle3d(position: {}, radius: {})",
            self.position, self.radius
        )
    }
}