#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrsfForm {
    Identity,
    Rotation,
    Translation,
    PointMirror,
    Ax1Mirror,
    Ax2Mirror,
    Scale,
    CompoundTrsf,
    Other,
}

impl TrsfForm {
    pub fn as_str(&self) -> &str {
        match self {
            TrsfForm::Identity => "Identity",
            TrsfForm::Rotation => "Rotation",
            TrsfForm::Translation => "Translation",
            TrsfForm::PointMirror => "PointMirror",
            TrsfForm::Ax1Mirror => "Ax1Mirror",
            TrsfForm::Ax2Mirror => "Ax2Mirror",
            TrsfForm::Scale => "Scale",
            TrsfForm::CompoundTrsf => "CompoundTrsf",
            TrsfForm::Other => "Other",
        }
    }
}
