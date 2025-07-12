use crate::traits::FloatWithConst;
use crate::xyz::XYZ;

#[derive(Debug, Clone, Copy)]
pub struct Direction3d<T = f64> {
    pub xyz: XYZ<T>,
}

impl<T> std::fmt::Display for Direction3d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Direction3d({}, {}, {})",
            self.xyz.x, self.xyz.y, self.xyz.z
        )
    }
}

impl<T> Direction3d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Direction3d::default()
    }

    pub fn from_xyz(xyz: XYZ<T>) -> Self {
        let mut d = Direction3d { xyz };
        d.normalize();
        d
    }

    pub fn from_coords(x: T, y: T, z: T) -> Self {
        let mut d = Direction3d {
            xyz: XYZ::from_coords(x, y, z),
        };
        d.normalize();
        d
    }

    #[inline]
    fn normalize(&mut self) {
        let d = self.xyz.length();
        if d <= T::min_positive() {
            panic!("Cannot normalize zero length vector");
        }
        self.xyz.x /= d;
        self.xyz.y /= d;
        self.xyz.z /= d;
    }

    pub fn get_coords(&self) -> (T, T, T) {
        (self.xyz.x, self.xyz.y, self.xyz.z)
    }

    pub fn set_coords(&mut self, x: T, y: T, z: T) {
        self.xyz.x = x;
        self.xyz.y = y;
        self.xyz.z = z;
        self.normalize();
    }

    pub fn get_x(&self) -> T {
        self.xyz.x
    }

    pub fn get_y(&self) -> T {
        self.xyz.y
    }

    pub fn get_z(&self) -> T {
        self.xyz.z
    }

    pub fn get_xyz(&self) -> &XYZ<T> {
        &self.xyz
    }

    pub fn set_xyz(&mut self, xyz: XYZ<T>) {
        self.xyz = xyz;
        self.normalize();
    }

    pub fn is_equal(&self, other: &Self, tolerance: T) -> bool {
        self.xyz.is_equal(&other.xyz, tolerance)
    }
}

impl<T> From<(T, T, T)> for Direction3d<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn from(coords: (T, T, T)) -> Self {
        Direction3d {
            xyz: XYZ::from_coords(coords.0, coords.1, coords.2),
        }
    }
}

impl<T> Default for Direction3d<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn default() -> Self {
        Direction3d {
            xyz: XYZ::from_coords(
                T::from(1.0).unwrap(),
                T::from(0.0).unwrap(),
                T::from(0.0).unwrap(),
            ),
        }
    }
}
