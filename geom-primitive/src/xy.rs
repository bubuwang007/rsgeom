use num_traits::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XY<T = f64> {
    pub x: T,
    pub y: T,
}

impl std::fmt::Display for XY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XY({}, {})", self.x, self.y)
    }
}

impl<T> XY<T>
where
    T: Copy + Default + Float,
{
    pub fn new() -> Self {
        XY {
            x: Default::default(),
            y: Default::default(),
        }
    }

    pub fn from_coordinates(x: T, y: T) -> Self {
        XY { x, y }
    }

    pub fn coord(&self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn set_coord(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }

    pub fn modulus(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn square_modulus(&self) -> T {
        self.x * self.x + self.y * self.y
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

    pub fn cross(&self, other: &Self) -> T {
        self.x * other.y - self.y * other.x
    }

    pub fn cross_abs(&self, other: &Self) -> T {
        self.cross(other).abs()
    }

    pub fn square_cross_abs(&self, other: &Self) -> T {
        self.cross(other).powi(2)
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(&mut self) {
        let modulus: T = self.modulus();
        if modulus > T::zero() {
            self.x = self.x / modulus;
            self.y = self.y / modulus;
        }
    }

    pub fn normalize_new(&self) -> Self {
        let mut result = *self;
        result.normalize();
        result
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

    // a1 * xy1 + a2 * xy2
    pub fn set_linear_form_2(&mut self, a1: T, xy1: &XY<T>, a2: T, xy2: &XY<T>) {
        self.x = a1 * xy1.x + a2 * xy2.x;
        self.y = a1 * xy1.y + a2 * xy2.y;
    }

    // a1 * xy1 + xy2
    pub fn set_linear_form_2a(&mut self, a1: T, xy1: &XY<T>, xy2: &XY<T>) {
        self.x = a1 * xy1.x + xy2.x;
        self.y = a1 * xy1.y + xy2.y;
    }

    // xy1 + xy2
    pub fn set_linear_form_2b(&mut self, xy1: &XY<T>, xy2: &XY<T>) {
        self.x = xy1.x + xy2.x;
        self.y = xy1.y + xy2.y;
    }

    // a1 * xy1 + a2 * xy2 + xy3
    pub fn set_linear_form_3(&mut self, a1: T, xy1: &XY<T>, a2: T, xy2: &XY<T>, xy3: &XY<T>) {
        self.x = a1 * xy1.x + a2 * xy2.x + xy3.x;
        self.y = a1 * xy1.y + a2 * xy2.y + xy3.y;
    }
}

use std::ops::{Index, IndexMut};

impl<T> Index<usize> for XY<T>
where
    T: Copy + Default + Float,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for XY<T>
where
    T: Copy + Default + Float,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// XY = -&XY
impl<T> Neg for &XY<T>
where
    T: Copy + Default + Float,
{
    type Output = XY<T>;

    fn neg(self) -> Self::Output {
        self.reverse_new()
    }
}

// XY += &XY
impl<T> AddAssign<&XY<T>> for XY<T>
where
    T: Copy + Default + Float,
{
    fn add_assign(&mut self, other: &XY<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

// XY = &XY + &XY
impl<T> Add<&XY<T>> for &XY<T>
where
    T: Copy + Default + Float,
{
    type Output = XY<T>;

    fn add(self, other: &XY<T>) -> Self::Output {
        XY {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// XY += T
impl<T> AddAssign<T> for XY<T>
where
    T: Copy + Default + Float,
{
    fn add_assign(&mut self, other: T) {
        self.x = self.x + other;
        self.y = self.y + other;
    }
}

// XY = &XY + T
impl<T> Add<T> for XY<T>
where
    T: Copy + Default + Float,
{
    type Output = XY<T>;

    fn add(self, other: T) -> Self::Output {
        XY {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

// XY -= &XY
impl<T> SubAssign<&XY<T>> for XY<T>
where
    T: Copy + Default + Float,
{
    fn sub_assign(&mut self, other: &XY<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

// XY = &XY - &XY
impl<T> Sub<&XY<T>> for &XY<T>
where
    T: Copy + Default + Float,
{
    type Output = XY<T>;

    fn sub(self, other: &XY<T>) -> Self::Output {
        XY {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// XY = &XY - T
impl<T> Sub<T> for &XY<T>
where
    T: Copy + Default + Float,
{
    type Output = XY<T>;

    fn sub(self, other: T) -> Self::Output {
        XY {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

// XY -= T
impl<T> SubAssign<T> for XY<T>
where
    T: Copy + Default + Float,
{
    fn sub_assign(&mut self, other: T) {
        self.x = self.x + other;
        self.y = self.y + other;
    }
}



impl Div<&XY> for &XY {
    type Output = XY;

    fn div(self, other: &XY) -> Self::Output {
        XY {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}


impl DivAssign<&XY> for XY {
    fn div_assign(&mut self, other: &XY) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl Div<f64> for &XY {
    type Output = XY;

    fn div(self, other: f64) -> Self::Output {
        XY {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl DivAssign<f64> for XY {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
    }
}

impl MulAssign<&XY> for XY {
    fn mul_assign(&mut self, other: &XY) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl Mul<&XY> for &XY {
    type Output = XY;

    fn mul(self, other: &XY) -> Self::Output {
        XY {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f64> for &XY {
    type Output = XY;

    fn mul(self, other: f64) -> Self::Output {
        XY {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign<f64> for XY {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
    }
}

impl Mul<&XY> for f64 {
    type Output = XY;

    fn mul(self, other: &XY) -> Self::Output {
        XY {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

use crate::Matrix2d;

impl MulAssign<&Matrix2d> for XY {
    fn mul_assign(&mut self, other: &Matrix2d) {
        let x = self.x;
        let y = self.y;
        self.x = other[0][0] * x + other[0][1] * y;
        self.y = other[1][0] * x + other[1][1] * y;
    }
}

impl Mul<&Matrix2d> for &XY {
    type Output = XY;

    fn mul(self, other: &Matrix2d) -> Self::Output {
        let x = self.x;
        let y = self.y;
        XY {
            x: other[0][0] * x + other[0][1] * y,
            y: other[1][0] * x + other[1][1] * y,
        }
    }
}
