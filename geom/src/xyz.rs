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

impl<T> From<(T, T, T)> for XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn from(coords: (T, T, T)) -> Self {
        XYZ {
            x: coords.0,
            y: coords.1,
            z: coords.2,
        }
    }
}

impl<T> From<[T; 3]> for XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn from(coords: [T; 3]) -> Self {
        XYZ {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

impl<T> XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        let d = self.length();
        if d <= T::min_positive() {
            panic!("Cannot normalize zero length vector");
        }
        self.x /= d;
        self.y /= d;
        self.z /= d;
    }

    pub fn normalize_new(&self) -> Self {
        let mut new_xyz = *self;
        new_xyz.normalize();
        new_xyz
    }

    pub fn cross(&mut self, other: &Self) {
        let xres = self.y * other.z - self.z * other.y;
        let yres = self.z * other.x - self.x * other.z;
        self.z = self.x * other.y - self.y * other.x;
        self.x = xres;
        self.y = yres;
    }

    pub fn cross_new(&self, other: &Self) -> Self {
        XYZ {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn cross_magnitude(&self) -> T {
        let c = self.cross_new(self);
        (c.x * c.x + c.y * c.y + c.z * c.z).sqrt()
    }

    pub fn square_cross_magnitude(&self) -> T {
        let c = self.cross_new(self);
        c.x * c.x + c.y * c.y + c.z * c.z
    }

    pub fn cross_cross(&mut self, o1: &Self, o2: &Self) {
        let xres = self.y * (o1.x * o2.y - o1.y * o2.x) - self.z * (o1.z * o2.x - o1.x * o2.z);
        let yres = self.z * (o1.y * o2.z - o1.z * o2.y) - self.x * (o1.x * o2.y - o1.y * o2.x);
        self.z = self.x * (o1.z * o2.x - o1.x * o2.z) - self.y * (o1.y * o2.z - o1.z * o2.y);
        self.x = xres;
        self.y = yres;
    }

    pub fn cross_cross_new(&self, o1: &Self, o2: &Self) -> Self {
        let xres = self.y * (o1.x * o2.y - o1.y * o2.x) - self.z * (o1.z * o2.x - o1.x * o2.z);
        let yres = self.z * (o1.y * o2.z - o1.z * o2.y) - self.x * (o1.x * o2.y - o1.y * o2.x);
        let zres = self.x * (o1.z * o2.x - o1.x * o2.z) - self.y * (o1.y * o2.z - o1.z * o2.y);
        XYZ {
            x: xres,
            y: yres,
            z: zres,
        }
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn dot_cross(&self, o1: &Self, o2: &Self) -> T {
        let xres = o1.y * o2.z - o1.z * o2.y;
        let yres = o1.z * o2.x - o1.x * o2.z;
        let zres = o1.x * o2.y - o1.y * o2.x;
        self.x * xres + self.y * yres + self.z * zres
    }

    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn reverse_new(&self) -> Self {
        XYZ {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

use std::ops::Neg;
impl<T> Neg for &XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XYZ<T>;

    fn neg(self) -> Self::Output {
        self.reverse_new()
    }
}

use std::ops::{Add, AddAssign};
impl<T> AddAssign<&XYZ<T>> for XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn add_assign(&mut self, other: &XYZ<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl<T> AddAssign<T> for XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn add_assign(&mut self, other: T) {
        self.x = self.x + other;
        self.y = self.y + other;
        self.z = self.z + other;
    }
}

impl<T> Add<&XYZ<T>> for &XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XYZ<T>;

    fn add(self, other: &XYZ<T>) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Add<T> for &XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XYZ<T>;

    fn add(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

use std::ops::{Sub, SubAssign};
impl<T> SubAssign<&XYZ<T>> for XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn sub_assign(&mut self, other: &XYZ<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
}

impl<T> SubAssign<T> for XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn sub_assign(&mut self, other: T) {
        self.x = self.x - other;
        self.y = self.y - other;
        self.z = self.z - other;
    }
}

impl<T> Sub<&XYZ<T>> for &XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XYZ<T>;

    fn sub(self, other: &XYZ<T>) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Sub<T> for &XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XYZ<T>;

    fn sub(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

use std::ops::{Div, DivAssign};
impl<T> DivAssign<&XYZ<T>> for XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn div_assign(&mut self, other: &XYZ<T>) {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
        self.z = self.z / other.z;
    }
}

impl<T> DivAssign<T> for XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn div_assign(&mut self, other: T) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
    }
}

impl<T> Div<&XYZ<T>> for &XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XYZ<T>;

    fn div(self, other: &XYZ<T>) -> Self::Output {
        Self::Output {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T> Div<T> for &XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XYZ<T>;

    fn div(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

use std::ops::{Mul, MulAssign};
impl<T> MulAssign<&XYZ<T>> for XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn mul_assign(&mut self, other: &XYZ<T>) {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
    }
}

impl<T> MulAssign<T> for XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn mul_assign(&mut self, other: T) {
        self.x = self.x * other;
        self.y = self.y * other;
        self.z = self.z * other;
    }
}

impl<T> Mul<&XYZ<T>> for &XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XYZ<T>;

    fn mul(self, other: &XYZ<T>) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> Mul<T> for &XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XYZ<T>;

    fn mul(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<&XYZ<f64>> for f64 {
    type Output = XYZ<f64>;

    fn mul(self, other: &XYZ) -> Self::Output {
        Self::Output {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<&XYZ<f32>> for f32 {
    type Output = XYZ<f32>;

    fn mul(self, other: &XYZ<f32>) -> Self::Output {
        Self::Output {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl<T> Mul<&crate::Matrix3<T>> for &XYZ<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XYZ<T>;

    fn mul(self, other: &crate::Matrix3<T>) -> Self::Output {
        let x = self.x;
        let y = self.y;
        Self::Output {
            x: other.m[0][0] * x + other.m[0][1] * y + other.m[0][2] * self.z,
            y: other.m[1][0] * x + other.m[1][1] * y + other.m[1][2] * self.z,
            z: other.m[2][0] * x + other.m[2][1] * y + other.m[2][2] * self.z,
        }
    }
}
