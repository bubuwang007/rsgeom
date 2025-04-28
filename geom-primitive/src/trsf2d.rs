use crate::{Matrix2d, Point2d, TrsfForm, XY, Vector2d};

pub struct Trsf2d {
    scalefac: f64,
    form: TrsfForm,
    matrix: Matrix2d,
    trans: XY,
}

impl Trsf2d {
    pub fn new() -> Self {
        Trsf2d {
            scalefac: 1.0,
            form: TrsfForm::Identity,
            matrix: Matrix2d::new(),
            trans: XY::new(),
        }
    }

    // pub fn from_trsf()

    pub fn to_string(&self) -> String {
        format!(
            "Trsf2d(scale: {}, form: {:?}, matrix: {}, translation: {})",
            self.scalefac,
            self.form.as_str(),
            self.matrix.to_string(),
            self.trans.to_string()
        )
    }

    pub fn scalefac(&self) -> f64 {
        self.scalefac
    }

    pub fn set_scalefac(&mut self, scale: f64) {
        self.scalefac = scale;
    }

    pub fn form(&self) -> TrsfForm {
        self.form
    }

    pub fn set_form(&mut self, form: TrsfForm) {
        self.form = form;
    }

    pub fn matrix(&self) -> &Matrix2d {
        &self.matrix
    }

    pub fn matrix_mut(&mut self) -> &mut Matrix2d {
        &mut self.matrix
    }

    pub fn set_matrix(&mut self, matrix: Matrix2d) {
        self.matrix = matrix;
    }

    pub fn trans(&self) -> &XY {
        &self.trans
    }

    pub fn trans_mut(&mut self) -> &mut XY {
        &mut self.trans
    }

    pub fn set_trans(&mut self, translation: XY) {
        self.trans = translation;
    }

    pub fn set_mirror(&mut self, p: &Point2d) {
        self.form = TrsfForm::PointMirror;
        self.scalefac = -1.0;
        self.matrix.set_identity();
        self.set_trans(*p.xy());
        self.trans *= 2.0;
    }

    pub fn set_scale(&mut self, p: &Point2d, scale: f64) {
        self.form = TrsfForm::Scale;
        self.scalefac = scale;
        self.matrix.set_identity();
        self.set_trans(*p.xy());
        self.trans *= 1.0 - scale;
    }

    pub fn set_translation_by_vec(&mut self, vec: &Vector2d) {
        self.form = TrsfForm::Translation;
        self.scalefac = 1.0;
        self.matrix.set_identity();
        self.trans = *vec.xy();
    }

    


}
