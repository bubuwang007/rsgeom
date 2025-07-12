use crate::traits::FloatWithConst;
use crate::xy::XY;

#[derive(Debug, Clone, Copy)]
pub struct Direction2d<T = f64> {
    pub xy: XY<T>,
}

impl<T> std::fmt::Display for Direction2d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Direction2d({}, {})", self.xy.x, self.xy.y)
    }
}

impl<T> Direction2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Direction2d::default()
    }

    pub fn from_xy(xy: XY<T>) -> Self {
        let mut d = Direction2d { xy };
        d.normalize();
        d
    }

    pub fn from_coords(x: T, y: T) -> Self {
        let mut d = Direction2d {
            xy: XY::from_coords(x, y),
        };
        d.normalize();
        d
    }

    #[inline]
    fn normalize(&mut self) {
        self.xy.normalize();
    }

    pub fn get_coords(&self) -> (T, T) {
        (self.xy.x, self.xy.y)
    }

    pub fn set_coords(&mut self, x: T, y: T) {
        self.xy.x = x;
        self.xy.y = y;
        self.normalize();
    }

    pub fn get_x(&self) -> T {
        self.xy.x
    }

    pub fn get_y(&self) -> T {
        self.xy.y
    }

    pub fn get_xy(&self) -> &XY<T> {
        &self.xy
    }

    pub fn set_xy(&mut self, xy: XY<T>) {
        self.xy = xy;
        self.normalize();
    }

    pub fn is_equal(&self, other: &Self, tolerance: T) -> bool {
        self.xy.is_equal(&other.xy, tolerance)
    }
}

impl<T> From<(T, T)> for Direction2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn from(coords: (T, T)) -> Self {
        Direction2d::from_coords(coords.0, coords.1)
    }
}

impl<T> Default for Direction2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn default() -> Self {
        Direction2d {
            xy: XY::from_coords(T::from(1.0).unwrap(), T::from(0.0).unwrap()),
        }
    }
}
