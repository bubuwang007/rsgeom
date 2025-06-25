use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct Matrix2<T = f64> {
    pub m: [[T; 2]; 2],
}

impl<T> std::fmt::Display for Matrix2<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix2([{}, {}], [{}, {}])",
            self.m[0][0], self.m[0][1], self.m[1][0], self.m[1][1]
        )
    }
}

impl<T> Matrix2<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Matrix2 {
            m: [[T::default(); 2]; 2],
        }
    }

    pub fn from_array(m: [[T; 2]; 2]) -> Self {
        Matrix2 { m }
    }

    pub fn from_values(m00: T, m01: T, m10: T, m11: T) -> Self {
        Matrix2 {
            m: [[m00, m01], [m10, m11]],
        }
    }

    pub fn get_values(&self) -> [[T; 2]; 2] {
        self.m
    }

    pub fn set_values(&mut self, m00: T, m01: T, m10: T, m11: T) {
        self.m = [[m00, m01], [m10, m11]];
    }

    pub fn get_col(&self, col: usize) -> [T; 2] {
        if col < 2 {
            [self.m[0][col], self.m[1][col]]
        } else {
            panic!("Column index out of bounds");
        }
    }

    pub fn set_col(&mut self, col: usize, values: [T; 2]) {
        if col < 2 {
            self.m[0][col] = values[0];
            self.m[1][col] = values[1];
        } else {
            panic!("Column index out of bounds");
        }
    }

    pub fn get_row(&self, row: usize) -> [T; 2] {
        if row < 2 {
            self.m[row]
        } else {
            panic!("Row index out of bounds");
        }
    }

    pub fn set_row(&mut self, row: usize, values: [T; 2]) {
        if row < 2 {
            self.m[row] = values;
        } else {
            panic!("Row index out of bounds");
        }
    }

    pub fn get_diagonal(&self) -> [T; 2] {
        [self.m[0][0], self.m[1][1]]
    }

    pub fn set_diagonal(&mut self, values: [T; 2]) {
        self.m[0][0] = values[0];
        self.m[1][1] = values[1];
    }
}
