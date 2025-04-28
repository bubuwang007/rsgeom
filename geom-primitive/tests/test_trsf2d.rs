use geom_primitive::{Point2d, Trsf2d, TrsfForm};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let t1 = Trsf2d::new();
        assert_eq!(t1.scalefac(), 1.0);
        assert_eq!(t1.form(), TrsfForm::Identity);
        assert_eq!(t1.matrix().matrix(), [[0.0, 0.0], [0.0, 0.0]]);
        assert_eq!(t1.trans().coord(), (0.0, 0.0));

        // Test from_trsf
    }

    #[test]
    fn test_set_mirror() {
        let mut trsf1 = Trsf2d::new();
        let p = Point2d::from_coordinates(1.0, 2.0);
        trsf1.set_mirror(&p);

        assert_eq!(trsf1.form(), TrsfForm::PointMirror);
        assert_eq!(trsf1.scalefac(), -1.0);
        assert_eq!(trsf1.matrix().matrix(), [[1.0, 0.0], [0.0, 1.0]]);
        assert_eq!(trsf1.trans().coord(), (2.0, 4.0));
    }

    #[test]
    fn test_set_scale() {
        let mut trsf1 = Trsf2d::new();
        trsf1.set_scale(&Point2d::from_coordinates(1.0, 2.0), 2.0);
        assert_eq!(trsf1.scalefac(), 2.0);
        assert_eq!(trsf1.form(), TrsfForm::Scale);
        assert_eq!(trsf1.matrix().matrix(), [[1.0, 0.0], [0.0, 1.0]]);
        assert_eq!(trsf1.trans().coord(), (-1.0, -2.0));
    }
}
