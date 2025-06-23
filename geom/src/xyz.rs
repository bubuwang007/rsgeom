use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct XYZ<T = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> std::fmt::Display for XYZ<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XYZ({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T> XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        XYZ {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }

    pub fn from_coords(x: T, y: T, z: T) -> Self {
        XYZ { x, y, z }
    }

    pub fn get_coords(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    pub fn set_coords(&mut self, x: T, y: T, z: T) {
        self.x = x;
        self.y = y;
        self.z = z;
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

    pub fn get_z(&self) -> T {
        self.z
    }

    pub fn set_z(&mut self, z: T) {
        self.z = z;
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
        let val = self.z - other.z;
        if val.abs() > tolerance {
            return false;
        }
        true
    }
}
