use crate::Quaternion;

#[derive(Debug, Clone, Copy)]
pub struct QuaternionNlerp<T = f64> {
    pub start: Quaternion<T>,
    pub end: Quaternion<T>,
}
