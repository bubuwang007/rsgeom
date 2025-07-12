use crate::direction3d::Direction3d;
use crate::point3d::Point3d;
use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct Axis3d<T = f64> {
    pub location: Point3d<T>,
    pub direction: Direction3d<T>,
}

impl<T> std::fmt::Display for Axis3d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Axis3({}, {})", self.location, self.direction)
    }
}

impl<T> Axis3d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Axis3d {
            location: Point3d::new(),
            direction: Direction3d::new(),
        }
    }

    pub fn from_location_direction<P, D>(location: P, direction: D) -> Self
    where
        P: Into<Point3d<T>>,
        D: Into<Direction3d<T>>,
    {
        Axis3d {
            location: location.into(),
            direction: direction.into(),
        }
    }

    pub fn get_location(&self) -> &Point3d<T> {
        &self.location
    }

    pub fn set_location<P>(&mut self, location: P)
    where
        P: Into<Point3d<T>>,
    {
        self.location = location.into();
    }

    pub fn get_direction(&self) -> &Direction3d<T> {
        &self.direction
    }

    pub fn set_direction<D>(&mut self, direction: D)
    where
        D: Into<Direction3d<T>>,
    {
        self.direction = direction.into();
    }
}
