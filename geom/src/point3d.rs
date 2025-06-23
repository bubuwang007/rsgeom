use crate::traits::FloatWithConst;
use crate::xyz::XYZ;

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

    pub fn from_xyz(xyz: XYZ<T>) -> Self {
        Point3d { xyz }
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
    
    // TODO

}