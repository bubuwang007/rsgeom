use crate::Matrix2;
use crate::TrsfForm;
use crate::XY;

#[derive(Debug, Clone, Copy)]
pub struct Trsf2d<T = f64> {
    pub matrix: Matrix2<T>,
    pub loc: XY,
    pub trsf_type: TrsfForm,
    pub scale: T,
}