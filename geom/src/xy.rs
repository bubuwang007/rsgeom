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

impl<T> From<[T; 2]> for XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn from(coords: [T; 2]) -> Self {
        XY {
            x: coords[0],
            y: coords[1],
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

    pub fn squared_length(&self) -> T {
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

use std::ops::Neg;
impl<T> Neg for &XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XY<T>;

    fn neg(self) -> Self::Output {
        self.reverse_new()
    }
}

use std::ops::{Add, AddAssign};
impl<T> AddAssign<&XY<T>> for XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn add_assign(&mut self, other: &XY<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<T> AddAssign<T> for XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn add_assign(&mut self, other: T) {
        self.x = self.x + other;
        self.y = self.y + other;
    }
}

impl<T> Add<&XY<T>> for &XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XY<T>;

    fn add(self, other: &XY<T>) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Add<T> for &XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XY<T>;

    fn add(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

use std::ops::{Sub, SubAssign};
impl<T> SubAssign<&XY<T>> for XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn sub_assign(&mut self, other: &XY<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl<T> SubAssign<T> for XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn sub_assign(&mut self, other: T) {
        self.x = self.x - other;
        self.y = self.y - other;
    }
}

impl<T> Sub<&XY<T>> for &XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XY<T>;

    fn sub(self, other: &XY<T>) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Sub<T> for &XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XY<T>;

    fn sub(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

use std::ops::{Div, DivAssign};
impl<T> DivAssign<&XY<T>> for XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn div_assign(&mut self, other: &XY<T>) {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
    }
}

impl<T> DivAssign<T> for XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn div_assign(&mut self, other: T) {
        self.x = self.x / other;
        self.y = self.y / other;
    }
}

impl<T> Div<&XY<T>> for &XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XY<T>;

    fn div(self, other: &XY<T>) -> Self::Output {
        Self::Output {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> Div<T> for &XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XY<T>;

    fn div(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

use std::ops::{Mul, MulAssign};
impl<T> MulAssign<&XY<T>> for XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn mul_assign(&mut self, other: &XY<T>) {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
    }
}

impl<T> MulAssign<T> for XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn mul_assign(&mut self, other: T) {
        self.x = self.x * other;
        self.y = self.y * other;
    }
}

impl<T> Mul<&XY<T>> for &XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XY<T>;

    fn mul(self, other: &XY<T>) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Mul<T> for &XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XY<T>;

    fn mul(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<&XY<f64>> for f64 {
    type Output = XY<f64>;

    fn mul(self, other: &XY) -> Self::Output {
        Self::Output {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl Mul<&XY<f32>> for f32 {
    type Output = XY<f32>;

    fn mul(self, other: &XY<f32>) -> Self::Output {
        Self::Output {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl<T> Mul<&crate::Matrix2<T>> for &XY<T>
where
    T: Copy + Default + FloatWithConst,
{
    type Output = XY<T>;

    fn mul(self, other: &crate::Matrix2<T>) -> Self::Output {
        let x = self.x;
        let y = self.y;
        Self::Output {
            x: other.m[0][0] * x + other.m[0][1] * y,
            y: other.m[1][0] * x + other.m[1][1] * y,
        }
    }
}
