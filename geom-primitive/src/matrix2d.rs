#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix2d {
    m: [[f64; 2]; 2],
}

use crate::xy::XY;

impl Matrix2d {
    pub fn new() -> Self {
        Matrix2d { m: [[0.0; 2]; 2] }
    }

    pub fn from_array(m: [[f64; 2]; 2]) -> Self {
        Matrix2d { m }
    }

    pub fn from_coordinates(v1: f64, v2: f64, v3: f64, v4: f64) -> Self {
        Matrix2d {
            m: [[v1, v2], [v3, v4]],
        }
    }

    pub fn col(&self, index: usize) -> [f64; 2] {
        [self.m[0][index], self.m[1][index]]
    }

    pub fn col_xy(&self, index: usize) -> XY {
        XY::from_coordinates(self.m[0][index], self.m[1][index])
    }

    pub fn row(&self, index: usize) -> [f64; 2] {
        self.m[index]
    }

    pub fn row_xy(&self, index: usize) -> XY {
        XY::from_coordinates(self.m[index][0], self.m[index][1])
    }

    pub fn set_col(&mut self, index: usize, col: [f64; 2]) {
        self.m[0][index] = col[0];
        self.m[1][index] = col[1];
    }

    pub fn set_col_xy(&mut self, index: usize, xy: XY) {
        self.m[0][index] = xy.x;
        self.m[1][index] = xy.y;
    }

    pub fn set_row(&mut self, index: usize, row: [f64; 2]) {
        self.m[index] = row;
    }

    pub fn set_row_xy(&mut self, index: usize, xy: XY) {
        self.m[index][0] = xy.x;
        self.m[index][1] = xy.y;
    }

    pub fn set_matrix(&mut self, m: [[f64; 2]; 2]) {
        self.m = m;
    }

    pub fn set_matrix_xy(&mut self, m: [XY; 2]) {
        self.m[0][0] = m[0].x;
        self.m[0][1] = m[0].y;
        self.m[1][0] = m[1].x;
        self.m[1][1] = m[1].y;
    }

    pub fn diagonal(&self) -> [f64; 2] {
        [self.m[0][0], self.m[1][1]]
    }

    pub fn diagonal_xy(&self) -> XY {
        XY::from_coordinates(self.m[0][0], self.m[1][1])
    }

    pub fn set_diagonal(&mut self, diag: [f64; 2]) {
        self.m[0][0] = diag[0];
        self.m[1][1] = diag[1];
    }

    pub fn set_diagonal_xy(&mut self, diag: XY) {
        self.m[0][0] = diag.x;
        self.m[1][1] = diag.y;
    }

    pub fn set_identity(&mut self) {
        self.m[0][0] = 1.0;
        self.m[1][1] = 1.0;
        self.m[0][1] = 0.0;
        self.m[1][0] = 0.0;
    }

    pub fn to_string(&self) -> String {
        format!(
            "Matrix2d[[{}, {}], [{}, {}]]",
            self.m[0][0], self.m[0][1], self.m[1][0], self.m[1][1]
        )
    }

    pub fn set_rotation(&mut self, angle: f64) {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        self.m[0][0] = cos_angle;
        self.m[0][1] = -sin_angle;
        self.m[1][0] = sin_angle;
        self.m[1][1] = cos_angle;
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.m[0][0] = scale;
        self.m[1][1] = scale;
        self.m[0][1] = 0.0;
        self.m[1][0] = 0.0;
    }

    pub fn determinant(&self) -> f64 {
        self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0]
    }

    pub fn is_singular(&self) -> bool {
        self.determinant().abs() < f64::MIN_POSITIVE
    }

}

use std::ops::{Index, IndexMut};

impl Index<usize> for Matrix2d {
    type Output = [f64; 2];

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}

impl IndexMut<usize> for Matrix2d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}
