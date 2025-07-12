use crate::direction2d::Direction2d;
use crate::point2d::Point2d;
use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct Axis2d<T = f64> {
    pub location: Point2d<T>,
    pub direction: Direction2d<T>,
}

impl<T> std::fmt::Display for Axis2d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Axis2({}, {})", self.location, self.direction)
    }
}

impl<T> Axis2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Axis2d {
            location: Point2d::new(),
            direction: Direction2d::new(),
        }
    }

    pub fn from_location_direction<P, D>(location: P, direction: D) -> Self
    where
        P: Into<Point2d<T>>,
        D: Into<Direction2d<T>>,
    {
        Axis2d {
            location: location.into(),
            direction: direction.into(),
        }
    }

    pub fn get_location(&self) -> &Point2d<T> {
        &self.location
    }

    pub fn set_location<P>(&mut self, location: P)
    where
        P: Into<Point2d<T>>,
    {
        self.location = location.into();
    }

    pub fn get_direction(&self) -> &Direction2d<T> {
        &self.direction
    }

    pub fn set_direction<D>(&mut self, direction: D)
    where
        D: Into<Direction2d<T>>,
    {
        self.direction = direction.into();
    }
}
