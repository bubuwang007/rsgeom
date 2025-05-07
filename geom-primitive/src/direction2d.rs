use crate::xy::XY;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction2d<T = f64> {
    xy: XY<T>,
}