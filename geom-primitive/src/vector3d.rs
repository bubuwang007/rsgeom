use crate::xyz::XYZ;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3d<T = f64> {
    xy: XYZ<T>,
}