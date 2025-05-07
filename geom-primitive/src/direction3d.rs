use crate::xyz::XYZ;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction3d<T = f64> {
    xyz: XYZ<T>,
}