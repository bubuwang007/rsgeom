use crate::Quaternion;

#[derive(Debug, Clone, Copy)]
pub struct QuaternionSlerp<T = f64> {
    pub start: Quaternion<T>,
    pub end: Quaternion<T>,
    pub omega: T,
}
