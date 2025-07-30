use crate::CoordinateSystem2d;

#[derive(Debug, Clone, Copy)]
pub struct Circle2d<T = f64> {
    pub position: CoordinateSystem2d<T>,
    pub radius: T,
}

impl<T> std::fmt::Display for Circle2d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Circle2d(position: {}, radius: {})",
            self.position, self.radius
        )
    }
}