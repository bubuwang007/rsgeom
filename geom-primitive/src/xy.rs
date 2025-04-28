#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XY {
    pub x: f64,
    pub y: f64,
}

impl XY {
    pub fn new() -> Self {
        XY { x: 0.0, y: 0.0 }
    }

    pub fn from_coordinates(x: f64, y: f64) -> Self {
        XY { x, y }
    }

    pub fn to_string(&self) -> String {
        format!("XY({}, {})", self.x, self.y)
    }

    pub fn coord(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn set_coord(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn modulus(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn square_modulus(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn is_equal(&self, other: &Self, tolerance: f64) -> bool {
        let val: f64 = self.x - other.x;
        if val.abs() > tolerance {
            return false;
        }
        let val = self.y - other.y;
        if val.abs() > tolerance {
            return false;
        }
        true
    }

    pub fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn cross_abs(&self, other: &Self) -> f64 {
        self.cross(other).abs()
    }

    pub fn square_cross_abs(&self, other: &Self) -> f64 {
        self.cross(other).powi(2)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(&mut self) {
        let modulus = self.modulus();
        if modulus > 0.0 {
            self.x /= modulus;
            self.y /= modulus;
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
    pub fn set_linear_form_2(&mut self, a1: f64, xy1: &XY, a2: f64, xy2: &XY) {
        self.x = a1 * xy1.x + a2 * xy2.x;
        self.y = a1 * xy1.y + a2 * xy2.y;
    }

    // a1 * xy1 + xy2
    pub fn set_linear_form_2a(&mut self, a1: f64, xy1: &XY, xy2: &XY) {
        self.x = a1 * xy1.x + xy2.x;
        self.y = a1 * xy1.y + xy2.y;
    }

    // xy1 + xy2
    pub fn set_linear_form_2b(&mut self, xy1: &XY, xy2: &XY) {
        self.x = xy1.x + xy2.x;
        self.y = xy1.y + xy2.y;
    }

    // a1 * xy1 + a2 * xy2 + xy3
    pub fn set_linear_form_3(&mut self, a1: f64, xy1: &XY, a2: f64, xy2: &XY, xy3: &XY) {
        self.x = a1 * xy1.x + a2 * xy2.x + xy3.x;
        self.y = a1 * xy1.y + a2 * xy2.y + xy3.y;
    }
}

use std::ops::{Index, IndexMut};

impl Index<usize> for XY {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for XY {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Neg for &XY {
    type Output = XY;

    fn neg(self) -> Self::Output {
        self.reverse_new()
    }
}

impl AddAssign<&XY> for XY {
    fn add_assign(&mut self, other: &XY) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add<&XY> for &XY {
    type Output = XY;

    fn add(self, other: &XY) -> Self::Output {
        XY {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<f64> for XY {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
    }
}

impl Add<f64> for XY {
    type Output = XY;

    fn add(self, other: f64) -> Self::Output {
        XY {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl SubAssign<&XY> for XY {
    fn sub_assign(&mut self, other: &XY) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Sub<&XY> for &XY {
    type Output = XY;

    fn sub(self, other: &XY) -> Self::Output {
        XY {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<f64> for &XY {
    type Output = XY;

    fn sub(self, other: f64) -> Self::Output {
        XY {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl SubAssign<f64> for XY {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
    }
}

impl DivAssign<&XY> for XY {
    fn div_assign(&mut self, other: &XY) {
        self.x /= other.x;
        self.y /= other.y;
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

// Todo Mul Matrix2d
