use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct XY<T = f64> {
    pub x: T,
    pub y: T,
}

impl<T> std::fmt::Display for XY<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XY({}, {})", self.x, self.y)
    }
}

impl<T> XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        XY {
            x: Default::default(),
            y: Default::default(),
        }
    }

    pub fn from_coords(x: T, y: T) -> Self {
        XY { x, y }
    }

    pub fn get_coords(&self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn set_coords(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }

    pub fn get_x(&self) -> T {
        self.x
    }

    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }

    pub fn get_y(&self) -> T {
        self.y
    }

    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }

    pub fn is_equal(&self, other: &Self, tolerance: T) -> bool {
        let val: T = self.x - other.x;
        if val.abs() > tolerance {
            return false;
        }
        let val = self.y - other.y;
        if val.abs() > tolerance {
            return false;
        }
        true
    }
}

impl<T> From<(T, T)> for XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn from(coords: (T, T)) -> Self {
        XY {
            x: coords.0,
            y: coords.1,
        }
    }
}

impl<T> XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn square_length(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(&mut self) {
        let d = self.length();
        if d <= T::min_positive() {
            panic!("Cannot normalize zero length vector");
        }
        self.x /= d;
        self.y /= d;
    }

    pub fn normalize_new(&self) -> Self {
        let mut new_xy = *self;
        new_xy.normalize();
        new_xy
    }

    pub fn cross(&self, other: &Self) -> T {
        self.x * other.y - self.y * other.x
    }

    pub fn cross_abs(&self, other: &Self) -> T {
        self.cross(other).abs()
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    pub fn reverse_new(&self) -> Self {
        let mut result = *self;
        result.reverse();
        result
    }
}
