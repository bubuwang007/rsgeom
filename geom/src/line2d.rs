use crate::Axis2d;

#[derive(Debug, Clone, Copy)]
pub struct Line2d<T = f64> {
    pub pos: Axis2d<T>,
}