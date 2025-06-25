use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct Matrix3<T = f64> {
    pub m: [[T; 3]; 3],
}
