use crate::traits::FloatWithConst;
use crate::XYZ;

#[derive(Debug, Clone, Copy)]
pub struct Point3d<T = f64> {
    xyz: XYZ<T>,
}

impl<T> std::fmt::Display for Point3d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point3d({}, {}, {})", self.xyz.x, self.xyz.y, self.xyz.z)
    }
}

impl<T> Point3d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Point3d { xyz: XYZ::new() }
    }

    pub fn from_xyz<X>(xyz: X) -> Self
    where
        X: Into<XYZ<T>>,
    {
        Point3d { xyz: xyz.into() }
    }

    pub fn from_coords(x: T, y: T, z: T) -> Self {
        Point3d {
            xyz: XYZ::from_coords(x, y, z),
        }
    }

    pub fn get_coords(&self) -> (T, T, T) {
        (self.xyz.x, self.xyz.y, self.xyz.z)
    }

    pub fn set_coords(&mut self, x: T, y: T, z: T) {
        self.xyz.x = x;
        self.xyz.y = y;
        self.xyz.z = z;
    }

    pub fn get_x(&self) -> T {
        self.xyz.x
    }

    pub fn set_x(&mut self, x: T) {
        self.xyz.x = x;
    }

    pub fn get_y(&self) -> T {
        self.xyz.y
    }

    pub fn set_y(&mut self, y: T) {
        self.xyz.y = y;
    }

    pub fn get_z(&self) -> T {
        self.xyz.z
    }

    pub fn set_z(&mut self, z: T) {
        self.xyz.z = z;
    }

    pub fn get_xyz(&self) -> XYZ<T> {
        self.xyz
    }

    pub fn set_xyz(&mut self, xyz: XYZ<T>) {
        self.xyz = xyz;
    }

    pub fn is_equal(&self, other: &Self, tolerance: T) -> bool {
        self.xyz.is_equal(&other.xyz, tolerance)
    }
}

impl<T> From<(T, T, T)> for Point3d<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn from(coords: (T, T, T)) -> Self {
        Point3d::from_coords(coords.0, coords.1, coords.2)
    }
}
