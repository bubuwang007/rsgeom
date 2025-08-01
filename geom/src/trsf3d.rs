use crate::Matrix3;
use crate::TrsfForm;
use crate::XYZ;

#[derive(Debug, Clone, Copy)]
pub struct Trsf3d<T = f64> {
    pub matrix: Matrix3<T>,
    pub loc: XYZ,
    pub trsf_type: TrsfForm,
    pub scale: T,
}
