use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct Matrix3<T = f64> {
    pub m: [[T; 3]; 3],
}

impl<T> std::fmt::Display for Matrix3<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix3([[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]])",
            self.m[0][0],
            self.m[0][1],
            self.m[0][2],
            self.m[1][0],
            self.m[1][1],
            self.m[1][2],
            self.m[2][0],
            self.m[2][1],
            self.m[2][2]
        )
    }
}

impl<T> Matrix3<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Matrix3 {
            m: [[T::default(); 3]; 3],
        }
    }

    pub fn from_array(m: [[T; 3]; 3]) -> Self {
        Matrix3 { m }
    }

    pub fn from_values(
        m00: T,
        m01: T,
        m02: T,
        m10: T,
        m11: T,
        m12: T,
        m20: T,
        m21: T,
        m22: T,
    ) -> Self {
        Matrix3 {
            m: [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]],
        }
    }

    pub fn get_values(&self) -> [[T; 3]; 3] {
        self.m
    }

    pub fn set_values(
        &mut self,
        m00: T,
        m01: T,
        m02: T,
        m10: T,
        m11: T,
        m12: T,
        m20: T,
        m21: T,
        m22: T,
    ) {
        self.m = [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]];
    }

    pub fn get_col(&self, col: usize) -> [T; 3] {
        if col < 3 {
            [self.m[0][col], self.m[1][col], self.m[2][col]]
        } else {
            panic!("Column index out of bounds");
        }
    }

    pub fn set_col(&mut self, col: usize, values: [T; 3]) {
        if col < 3 {
            self.m[0][col] = values[0];
            self.m[1][col] = values[1];
            self.m[2][col] = values[2];
        } else {
            panic!("Column index out of bounds");
        }
    }

    pub fn get_row(&self, row: usize) -> [T; 3] {
        if row < 3 {
            self.m[row]
        } else {
            panic!("Row index out of bounds");
        }
    }

    pub fn set_row(&mut self, row: usize, values: [T; 3]) {
        if row < 3 {
            self.m[row] = values;
        } else {
            panic!("Row index out of bounds");
        }
    }

    pub fn get_diagonal(&self) -> [T; 3] {
        [self.m[0][0], self.m[1][1], self.m[2][2]]
    }

    pub fn set_diagonal(&mut self, values: [T; 3]) {
        self.m[0][0] = values[0];
        self.m[1][1] = values[1];
        self.m[2][2] = values[2];
    }

    pub fn is_equal(&self, other: &Self, tolerance: T) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if (self.m[i][j] - other.m[i][j]).abs() > tolerance {
                    return false;
                }
            }
        }
        true
    }
}

impl<T> From<[[T; 3]; 3]> for Matrix3<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn from(m: [[T; 3]; 3]) -> Self {
        Matrix3::from_array(m)
    }
}
