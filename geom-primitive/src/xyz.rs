#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl XYZ {
    pub fn new() -> Self {
        XYZ { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn from_coordinates(x: f64, y: f64, z: f64) -> Self {
        XYZ { x, y, z }
    }

    pub fn to_string(&self) -> String {
        format!("XYZ({}, {}, {})", self.x, self.y, self.z)
    }

    pub fn coord(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn set_coord(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn modulus(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn square_modulus(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
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
        let val = self.z - other.z;
        if val.abs() > tolerance {
            return false;
        }
        true
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