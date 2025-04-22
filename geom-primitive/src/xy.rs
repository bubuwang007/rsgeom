use std::ops::{Index, IndexMut};

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

    pub fn change_coord(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds"),
        }
    }

    pub fn coord(&mut self, x: f64, y: f64) {
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

}

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
