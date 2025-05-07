#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3d<T = f64> {
    m: [[T; 3]; 3],
}
