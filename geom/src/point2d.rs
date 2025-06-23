use crate::traits::FloatWithConst;
use crate::xy::XY;

#[derive(Debug, Clone, Copy)]
pub struct Point2d<T = f64> {
    pub xy: XY<T>,
}

impl<T> std::fmt::Display for Point2d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point2d({}, {})", self.xy.x, self.xy.y)
    }
}

impl<T> Point2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Point2d { xy: XY::new() }
    }

    pub fn from_xy(xy: XY<T>) -> Self {
        Point2d { xy }
    }

    pub fn from_coords(x: T, y: T) -> Self {
        Point2d {
            xy: XY::from_coords(x, y),
        }
    }

    pub fn get_coords(&self) -> (T, T) {
        (self.xy.x, self.xy.y)
    }

    pub fn set_coords(&mut self, x: T, y: T) {
        self.xy.x = x;
        self.xy.y = y;
    }

    pub fn get_x(&self) -> T {
        self.xy.x
    }

    pub fn set_x(&mut self, x: T) {
        self.xy.x = x;
    }

    pub fn get_y(&self) -> T {
        self.xy.y
    }

    pub fn set_y(&mut self, y: T) {
        self.xy.y = y;
    }

    pub fn get_xy(&self) -> XY<T> {
        self.xy
    }

    pub fn set_xy(&mut self, xy: XY<T>) {
        self.xy = xy;
    }

    pub fn is_equal(&self, other: &Self, tolerance: T) -> bool {
        self.xy.is_equal(&other.xy, tolerance)
    }
}
