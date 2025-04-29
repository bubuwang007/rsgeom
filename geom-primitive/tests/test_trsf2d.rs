use geom_primitive::{Point2d, Trsf2d, TrsfForm, Vector2d};

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

    #[test]
    fn test_translation() {
        let v = Vector2d::from_coordinates(3.0, 4.0);
        let mut trsf1 = Trsf2d::new();
        trsf1.set_translation_by_vec(&v);
        assert_eq!(trsf1.scalefac(), 1.0);
        assert_eq!(trsf1.form(), TrsfForm::Translation);
        assert_eq!(trsf1.matrix().matrix(), [[1.0, 0.0], [0.0, 1.0]]);
        assert_eq!(trsf1.trans().coord(), (3.0, 4.0));

        let p1 = Point2d::from_coordinates(1.0, 2.0);
        let p2 = Point2d::from_coordinates(3.0, 4.0);
        let mut trsf2 = Trsf2d::new();
        trsf2.set_translation_by_2points(&p1, &p2);
        assert_eq!(trsf2.scalefac(), 1.0);
        assert_eq!(trsf2.form(), TrsfForm::Translation);
        assert_eq!(trsf2.matrix().matrix(), [[1.0, 0.0], [0.0, 1.0]]);
        assert_eq!(trsf2.trans().coord(), (2.0, 2.0));
    }

    #[test]
    fn test_set_rotation(){
        let mut trsf = Trsf2d::new();
        let p = Point2d::from_coordinates(1.0, 2.0);
        let angle = 1.57;
        trsf.set_rotation(&p, angle);
        assert_eq!(trsf.form(), TrsfForm::Rotation);
        assert_eq!(trsf.scalefac(), 1.0);
        assert_eq!(trsf.matrix().matrix(), [[0.0, -1.0], [1.0, 0.0]]);
        // todo
    }

    // #[test]
    // fn test_rotation_part() {
    //     let mut trsf1 = Trsf2d::new();
    //     trsf1.set_rotation(Point2d::from_coordinates(1.0, 2.0), 1.57);

    // }
}
