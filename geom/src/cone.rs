use crate::GeneralCoordinateSystem3d;

#[derive(Debug, Clone, Copy)]
pub struct Cone<T = f64> {
    pub position: GeneralCoordinateSystem3d<T>,
    pub radius: T,
    pub semi_angle: T,
}

impl<T> std::fmt::Display for Cone<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cone(position: {}, radius: {}, semi_angle: {})",
            self.position, self.radius, self.semi_angle
        )
    }
}