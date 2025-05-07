use crate::xy::XY;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix2d {
    m: [[f64; 2]; 2],
}

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

    pub fn matrix(&self) -> [[f64; 2]; 2] {
        self.m
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

    pub fn invert(&mut self) -> Result<(), &'static str> {
        let det = self.determinant();
        let nm = self.m;

        if det.abs() <= f64::MIN_POSITIVE {
            return Err("Matrix is singular and cannot be inverted");
        }
        let a_det = 1.0 / det;
        self.m[0][0] = nm[1][1] * a_det;
        self.m[0][1] = -nm[0][1] * a_det;
        self.m[1][0] = -nm[1][0] * a_det;
        self.m[1][1] = nm[0][0] * a_det;
        Result::Ok(())
    }

    pub fn invert_new(&self) -> Result<Matrix2d, &'static str> {
        let mut m = *self;
        m.invert()?;
        Result::Ok(m)
    }

    pub fn matmul(&mut self, other: &Matrix2d) {
        let a = self.m[0][0] * other.m[0][0] + self.m[0][1] * other.m[1][0];
        let b = self.m[0][0] * other.m[0][1] + self.m[0][1] * other.m[1][1];
        let c = self.m[1][0] * other.m[0][0] + self.m[1][1] * other.m[1][0];
        let d = self.m[1][0] * other.m[0][1] + self.m[1][1] * other.m[1][1];

        self.m[0][0] = a;
        self.m[0][1] = b;
        self.m[1][0] = c;
        self.m[1][1] = d;
    }

    pub fn matmul_new(&self, other: &Matrix2d) -> Matrix2d {
        let mut m = *self;
        m.matmul(other);
        m
    }

    pub fn left_matmul(&mut self, other: &Matrix2d) {
        let a = other.m[0][0] * self.m[0][0] + other.m[0][1] * self.m[1][0];
        let b = other.m[0][0] * self.m[0][1] + other.m[0][1] * self.m[1][1];
        let c = other.m[1][0] * self.m[0][0] + other.m[1][1] * self.m[1][0];
        let d = other.m[1][0] * self.m[0][1] + other.m[1][1] * self.m[1][1];

        self.m[0][0] = a;
        self.m[0][1] = b;
        self.m[1][0] = c;
        self.m[1][1] = d;
    }

    pub fn left_matmul_new(&self, other: &Matrix2d) -> Matrix2d {
        let mut m = *self;
        m.left_matmul(other);
        m
    }

    pub fn powi(&mut self, n: i32) {
        self.m[0][0] = self.m[0][0].powi(n);
        self.m[0][1] = self.m[0][1].powi(n);
        self.m[1][0] = self.m[1][0].powi(n);
        self.m[1][1] = self.m[1][1].powi(n);
    }

    pub fn powi_new(&self, n: i32) -> Matrix2d {
        let mut m = *self;
        m.powi(n);
        m
    }

    pub fn powf(&mut self, n: f64) {
        self.m[0][0] = self.m[0][0].powf(n);
        self.m[0][1] = self.m[0][1].powf(n);
        self.m[1][0] = self.m[1][0].powf(n);
        self.m[1][1] = self.m[1][1].powf(n);
    }

    pub fn powf_new(&self, n: f64) -> Matrix2d {
        let mut m = *self;
        m.powf(n);
        m
    }

    pub fn matpowi(&mut self, n: i32) {
        match n {
            0 => self.set_identity(),
            1 => {}
            -1 => {
                self.invert().unwrap();
            }
            _ => {
                if n < 0 {
                    self.invert().unwrap();
                }
                let mut n_power = n.abs();
                n_power -= 1;
                let mut tmp_m = *self;
                loop {
                    if n_power % 2 == 1 {
                        self.matmul(&tmp_m);
                    }
                    if n_power == 1 {
                        break;
                    }
                    tmp_m.matmul(&tmp_m.clone());
                    n_power /= 2;
                }
            }
        }
    }

    pub fn matpowi_new(&self, n: i32) -> Matrix2d {
        let mut m = *self;
        m.matpowi(n);
        m
    }

    pub fn transpose(&mut self) {
        let tmp = self.m[0][1];
        self.m[0][1] = self.m[1][0];
        self.m[1][0] = tmp;
    }

    pub fn transpose_new(&self) -> Matrix2d {
        let mut m = *self;
        m.transpose();
        m
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

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Neg for &Matrix2d {
    type Output = Matrix2d;

    fn neg(self) -> Self::Output {
        Matrix2d {
            m: [
                [-self.m[0][0], -self.m[0][1]],
                [-self.m[1][0], -self.m[1][1]],
            ],
        }
    }
}

impl AddAssign<f64> for Matrix2d {
    fn add_assign(&mut self, other: f64) {
        self.m[0][0] += other;
        self.m[0][1] += other;
        self.m[1][0] += other;
        self.m[1][1] += other;
    }
}

impl AddAssign<&Matrix2d> for Matrix2d {
    fn add_assign(&mut self, other: &Matrix2d) {
        self.m[0][0] += other.m[0][0];
        self.m[0][1] += other.m[0][1];
        self.m[1][0] += other.m[1][0];
        self.m[1][1] += other.m[1][1];
    }
}

impl Add<f64> for &Matrix2d {
    type Output = Matrix2d;

    fn add(self, other: f64) -> Self::Output {
        Matrix2d {
            m: [
                [self.m[0][0] + other, self.m[0][1] + other],
                [self.m[1][0] + other, self.m[1][1] + other],
            ],
        }
    }
}

impl Add<&Matrix2d> for &Matrix2d {
    type Output = Matrix2d;

    fn add(self, other: &Matrix2d) -> Self::Output {
        Matrix2d {
            m: [
                [self.m[0][0] + other.m[0][0], self.m[0][1] + other.m[0][1]],
                [self.m[1][0] + other.m[1][0], self.m[1][1] + other.m[1][1]],
            ],
        }
    }
}

impl DivAssign<f64> for Matrix2d {
    fn div_assign(&mut self, other: f64) {
        self.m[0][0] /= other;
        self.m[0][1] /= other;
        self.m[1][0] /= other;
        self.m[1][1] /= other;
    }
}

impl DivAssign<&Matrix2d> for Matrix2d {
    fn div_assign(&mut self, other: &Matrix2d) {
        self.m[0][0] /= other.m[0][0];
        self.m[0][1] /= other.m[0][1];
        self.m[1][0] /= other.m[1][0];
        self.m[1][1] /= other.m[1][1];
    }
}

impl Div<f64> for &Matrix2d {
    type Output = Matrix2d;

    fn div(self, other: f64) -> Self::Output {
        Matrix2d {
            m: [
                [self.m[0][0] / other, self.m[0][1] / other],
                [self.m[1][0] / other, self.m[1][1] / other],
            ],
        }
    }
}

impl Div<&Matrix2d> for &Matrix2d {
    type Output = Matrix2d;

    fn div(self, other: &Matrix2d) -> Self::Output {
        Matrix2d {
            m: [
                [self.m[0][0] / other.m[0][0], self.m[0][1] / other.m[0][1]],
                [self.m[1][0] / other.m[1][0], self.m[1][1] / other.m[1][1]],
            ],
        }
    }
}

impl MulAssign<f64> for Matrix2d {
    fn mul_assign(&mut self, other: f64) {
        self.m[0][0] *= other;
        self.m[0][1] *= other;
        self.m[1][0] *= other;
        self.m[1][1] *= other;
    }
}

impl MulAssign<&Matrix2d> for Matrix2d {
    fn mul_assign(&mut self, other: &Matrix2d) {
        self.m[0][0] *= other.m[0][0];
        self.m[0][1] *= other.m[0][1];
        self.m[1][0] *= other.m[1][0];
        self.m[1][1] *= other.m[1][1];
    }
}

impl Mul<f64> for &Matrix2d {
    type Output = Matrix2d;

    fn mul(self, other: f64) -> Self::Output {
        Matrix2d {
            m: [
                [self.m[0][0] * other, self.m[0][1] * other],
                [self.m[1][0] * other, self.m[1][1] * other],
            ],
        }
    }
}

impl Mul<&Matrix2d> for f64 {
    type Output = Matrix2d;

    fn mul(self, other: &Matrix2d) -> Self::Output {
        return other * self;
    }
}

impl Mul<&Matrix2d> for Matrix2d {
    type Output = Matrix2d;

    fn mul(self, other: &Matrix2d) -> Self::Output {
        Matrix2d {
            m: [
                [self.m[0][0] * other.m[0][0], self.m[0][1] * other.m[0][1]],
                [self.m[1][0] * other.m[1][0], self.m[1][1] * other.m[1][1]],
            ],
        }
    }
}

impl SubAssign<f64> for Matrix2d {
    fn sub_assign(&mut self, other: f64) {
        self.m[0][0] -= other;
        self.m[0][1] -= other;
        self.m[1][0] -= other;
        self.m[1][1] -= other;
    }
}

impl SubAssign<&Matrix2d> for Matrix2d {
    fn sub_assign(&mut self, other: &Matrix2d) {
        self.m[0][0] -= other.m[0][0];
        self.m[0][1] -= other.m[0][1];
        self.m[1][0] -= other.m[1][0];
        self.m[1][1] -= other.m[1][1];
    }
}

impl Sub<f64> for &Matrix2d {
    type Output = Matrix2d;

    fn sub(self, other: f64) -> Self::Output {
        Matrix2d {
            m: [
                [self.m[0][0] - other, self.m[0][1] - other],
                [self.m[1][0] - other, self.m[1][1] - other],
            ],
        }
    }
}

impl Sub<&Matrix2d> for &Matrix2d {
    type Output = Matrix2d;

    fn sub(self, other: &Matrix2d) -> Self::Output {
        Matrix2d {
            m: [
                [self.m[0][0] - other.m[0][0], self.m[0][1] - other.m[0][1]],
                [self.m[1][0] - other.m[1][0], self.m[1][1] - other.m[1][1]],
            ],
        }
    }
}
