use crate::Axis3d;

#[derive(Debug, Clone, Copy)]
pub struct Line3d<T = f64> {
    pub pos: Axis3d<T>,
}