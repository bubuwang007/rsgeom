use crate::fconst::FloatWithConst;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn from_coordinates(x: T, y: T, z: T) -> Self {
        XYZ { x, y, z }
    }

    pub fn coord(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    pub fn set_coord(&mut self, x: T, y: T, z: T) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn modulus(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn square_modulus(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
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

    pub fn cross_abs(&self, other: &Self) -> T {
        let xres = self.y * other.z - self.z * other.y;
        let yres = self.z * other.x - self.x * other.z;
        let zres = self.x * other.y - self.y * other.x;
        (xres * xres + yres * yres + zres * zres).sqrt()
    }

    pub fn square_cross(&self, other: &Self) -> T {
        let xres = self.y * other.z - self.z * other.y;
        let yres = self.z * other.x - self.x * other.z;
        let zres = self.x * other.y - self.y * other.x;
        xres * xres + yres * yres + zres * zres
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

    pub fn normalize(&mut self) {
        let modulus: T = self.modulus();
        if modulus > T::zero() {
            self.x = self.x / modulus;
            self.y = self.y / modulus;
            self.z = self.z / modulus;
        }
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

    // a1 * xyz1 + a2 * xyz2
    pub fn set_linear_form_2(&mut self, a1: T, xyz1: &XYZ<T>, a2: T, xyz2: &XYZ<T>) {
        self.x = a1 * xyz1.x + a2 * xyz2.x;
        self.y = a1 * xyz1.y + a2 * xyz2.y;
        self.z = a1 * xyz1.z + a2 * xyz2.z;
    }

    // a1 * xyz1 + xyz2
    pub fn set_linear_form_2a(&mut self, a1: T, xyz1: &XYZ<T>, xyz2: &XYZ<T>) {
        self.x = a1 * xyz1.x + xyz2.x;
        self.y = a1 * xyz1.y + xyz2.y;
        self.z = a1 * xyz1.z + xyz2.z;
    }

    // xyz1 + xyz2
    pub fn set_linear_form_2b(&mut self, xyz1: &XYZ<T>, xyz2: &XYZ<T>) {
        self.x = xyz1.x + xyz2.x;
        self.y = xyz1.y + xyz2.y;
        self.z = xyz1.z + xyz2.z;
    }

    // a1 * xyz1 + a2 * xyz2 + xyz3
    pub fn set_linear_form_3(&mut self, a1: T, xyz1: &XYZ<T>, a2: T, xyz2: &XYZ<T>, xyz3: &XYZ<T>) {
        self.x = a1 * xyz1.x + a2 * xyz2.x + xyz3.x;
        self.y = a1 * xyz1.y + a2 * xyz2.y + xyz3.y;
        self.z = a1 * xyz1.z + a2 * xyz2.z + xyz3.z;
    }

    // a1 * xyz1 + a2 * xyz2 + a3 * xyz3
    pub fn set_linear_form_3a(
        &mut self,
        a1: T,
        xyz1: &XYZ<T>,
        a2: T,
        xyz2: &XYZ<T>,
        a3: T,
        xyz3: &XYZ<T>,
    ) {
        self.x = a1 * xyz1.x + a2 * xyz2.x + a3 * xyz3.x;
        self.y = a1 * xyz1.y + a2 * xyz2.y + a3 * xyz3.y;
        self.z = a1 * xyz1.z + a2 * xyz2.z + a3 * xyz3.z;
    }

    // a1 * xyz1 + a2 * xyz2 + a3 * xyz3 + xyz4
    pub fn set_linear_form_4(
        &mut self,
        a1: T,
        xyz1: &XYZ<T>,
        a2: T,
        xyz2: &XYZ<T>,
        a3: T,
        xyz3: &XYZ<T>,
        xyz4: &XYZ<T>,
    ) {
        self.x = a1 * xyz1.x + a2 * xyz2.x + a3 * xyz3.x + xyz4.x;
        self.y = a1 * xyz1.y + a2 * xyz2.y + a3 * xyz3.y + xyz4.y;
        self.z = a1 * xyz1.z + a2 * xyz2.z + a3 * xyz3.z + xyz4.z;
    }
}

use std::ops::{Index, IndexMut};

impl Index<usize> for XYZ {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for XYZ {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

use std::ops::Neg;

// XYZ = -XYZ
impl<T> Neg for &XYZ<T>
where
    T: Copy + FloatWithConst,
{
    type Output = XYZ<T>;

    fn neg(self) -> Self::Output {
        XYZ {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

use std::ops::{Add, AddAssign};

// XYZ += &XYZ
impl<T> AddAssign<&XYZ<T>> for XYZ<T>
where
    T: Copy + FloatWithConst,
{
    fn add_assign(&mut self, other: &XYZ<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

// XYZ += T
impl<T> AddAssign<T> for XYZ<T>
where
    T: Copy + FloatWithConst,
{
    fn add_assign(&mut self, other: T) {
        self.x = self.x + other;
        self.y = self.y + other;
        self.z = self.z + other;
    }
}

// XYZ = &XYZ + &XYZ
impl<T> Add<&XYZ<T>> for &XYZ<T>
where
    T: Copy + FloatWithConst,
{
    type Output = XYZ<T>;

    fn add(self, other: &XYZ<T>) -> Self::Output {
        XYZ {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// XYZ = &XYZ + T
impl<T> Add<T> for XYZ<T>
where
    T: Copy + FloatWithConst,
{
    type Output = XYZ<T>;

    fn add(self, other: T) -> Self::Output {
        XYZ {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

use std::ops::{Sub, SubAssign};

// XYZ -= &XYZ
impl<T> SubAssign<&XYZ<T>> for XYZ<T>
where
    T: Copy + FloatWithConst,
{
    fn sub_assign(&mut self, other: &XYZ<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
}

// XYZ -= T
impl<T> SubAssign<T> for XYZ<T>
where
    T: Copy + FloatWithConst,
{
    fn sub_assign(&mut self, other: T) {
        self.x = self.x - other;
        self.y = self.y - other;
        self.z = self.z - other;
    }
}

// XYZ = &XYZ - &XYZ
impl<T> Sub<&XYZ<T>> for &XYZ<T>
where
    T: Copy + FloatWithConst,
{
    type Output = XYZ<T>;

    fn sub(self, other: &XYZ<T>) -> Self::Output {
        XYZ {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// XYZ = &XYZ - T
impl<T> Sub<T> for &XYZ<T>
where
    T: Copy + FloatWithConst,
{
    type Output = XYZ<T>;

    fn sub(self, other: T) -> Self::Output {
        XYZ {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

use std::ops::{Div, DivAssign};

// XYZ /= T
impl<T> DivAssign<&XYZ<T>> for XYZ<T>
where
    T: Copy + FloatWithConst,
{
    fn div_assign(&mut self, other: &XYZ<T>) {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
        self.z = self.z / other.z;
    }
}

// XYZ /= T
impl<T> DivAssign<T> for XYZ<T>
where
    T: Copy + FloatWithConst,
{
    fn div_assign(&mut self, other: T) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
    }
}

// XYZ = &XYZ / &XYZ
impl<T> Div<&XYZ<T>> for &XYZ<T>
where
    T: Copy + FloatWithConst,
{
    type Output = XYZ<T>;

    fn div(self, other: &XYZ<T>) -> Self::Output {
        XYZ {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

// XYZ = &XYZ / T
impl<T> Div<T> for &XYZ<T>
where
    T: Copy + FloatWithConst,
{
    type Output = XYZ<T>;

    fn div(self, other: T) -> Self::Output {
        XYZ {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

use std::ops::{Mul, MulAssign};

// XYZ *= &XYZ
impl<T> MulAssign<&XYZ<T>> for XYZ<T>
where
    T: Copy + FloatWithConst,
{
    fn mul_assign(&mut self, other: &XYZ<T>) {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
    }
}

// XYZ *= T
impl<T> MulAssign<T> for XYZ<T>
where
    T: Copy + FloatWithConst,
{
    fn mul_assign(&mut self, other: T) {
        self.x = self.x * other;
        self.y = self.y * other;
        self.z = self.z * other;
    }
}

// TODO
// impl MulAssign<&crate::Matrix3d> for XYZ {
//     fn mul_assign(&mut self, other: &crate::Matrix2d) {
//         let x = self.x;
//         let y = self.y;
//         self.x = other[0][0] * x + other[0][1] * y;
//         self.y = other[1][0] * x + other[1][1] * y;
//     }
// }

// XYZ = &XYZ * &XYZ
impl<T> Mul<&XYZ<T>> for &XYZ<T>
where
    T: Copy + FloatWithConst,
{
    type Output = XYZ<T>;

    fn mul(self, other: &XYZ<T>) -> Self::Output {
        XYZ {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// XYZ = &XYZ * T
impl<T> Mul<T> for &XYZ<T>
where
    T: Copy + FloatWithConst,
{
    type Output = XYZ<T>;

    fn mul(self, other: T) -> Self::Output {
        XYZ {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

// XYZ = f64 * &XYZ
impl Mul<&XYZ> for f64 {
    type Output = XYZ;

    fn mul(self, other: &XYZ) -> Self::Output {
        XYZ {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

// XYZ = f32 * &XYZ
impl Mul<&XYZ<f32>> for f32 {
    type Output = XYZ<f32>;

    fn mul(self, other: &XYZ<f32>) -> Self::Output {
        XYZ {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

// TODO
// impl Mul<&crate::Matrix3d> for &XYZ {
//     type Output = XYZ;

//     fn mul(self, other: &crate::Matrix2d) -> Self::Output {
//         let x = self.x;
//         let y = self.y;
//         XYZ {
//             x: other[0][0] * x + other[0][1] * y,
//             y: other[1][0] * x + other[1][1] * y,
//         }
//     }
// }
