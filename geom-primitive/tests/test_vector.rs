use geom_primitive::{Point2d, Vector2d, XY};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let v1 = Vector2d::new();
        assert_eq!(v1.xy.x, 0.0);
        assert_eq!(v1.xy.y, 0.0);

        let v2 = Vector2d::from_coordinates(1.0, 2.0);
        assert_eq!(v2.xy.x, 1.0);
        assert_eq!(v2.xy.y, 2.0);

        let mut v3 = Vector2d::from_xy(XY::new());
        v3.xy.x = 3.0;
        v3.xy.y = 4.0;
        assert_eq!(v3.xy.x, 3.0);

        let p1 = Point2d::from_coordinates(10.0, 15.0);
        let p2 = Point2d::from_coordinates(20.0, 25.0);
        let v4 = Vector2d::from_points(&p1, &p2);
        assert_eq!(v4.xy.x, 10.0);
        assert_eq!(v4.xy.y, 10.0);
    }

    #[test]
    fn test_magnitude() {
        let v = Vector2d::from_coordinates(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
        assert_eq!(v.square_length(), 25.0);
    }

    #[test]
    fn test_angle_to_vector() {
        let v1 = Vector2d::from_coordinates(1.0, 0.0);
        let v2 = Vector2d::from_coordinates(0.0, 1.0);
        let v3 = Vector2d::from_coordinates(1.0, 1.0);
        let v4 = Vector2d::from_coordinates(-1.0, 1.0);
        assert_eq!(
            v1.angle_to_vector(&v2).unwrap(),
            std::f64::consts::FRAC_PI_2
        );
        assert_eq!(
            (v1.angle_to_vector(&v3).unwrap() - std::f64::consts::FRAC_PI_4).abs() < 1e-10,
            true
        );
        assert_eq!(
            (v1.angle_to_vector(&v4).unwrap() - std::f64::consts::PI * 0.75).abs() < 1e-10,
            true
        );
    }

    #[test]
    fn test_is_equal() {
        let v1 = Vector2d::from_coordinates(1.0, 2.0);
        let v2 = Vector2d::from_coordinates(1.0, 2.0);
        let v3 = Vector2d::from_coordinates(1.0, 3.0);
        assert_eq!(v1.is_equal(&v2, 1e-10, 1e-5).unwrap(), true);
        assert_eq!(v1.is_equal(&v3, 1e-10, 1e-5).unwrap(), false);
    }

    #[test]
    fn test_is_orthogonal() {
        let v1 = Vector2d::from_coordinates(1.0, 0.0);
        let v2 = Vector2d::from_coordinates(0.0, 1.0);
        let v3 = Vector2d::from_coordinates(1.0, 1.0);
        assert_eq!(v1.is_orthogonal(&v2, 1e-7).unwrap(), true);
        assert_eq!(v1.is_orthogonal(&v3, 1e-7).unwrap(), false);
    }

    #[test]
    fn test_is_opposite() {
        let v1 = Vector2d::from_coordinates(1.0, 0.0);
        let v2 = Vector2d::from_coordinates(-1.0, 0.0);
        let v3 = Vector2d::from_coordinates(1.0, 1.0);
        assert_eq!(v1.is_opposite(&v2, 1e-7).unwrap(), true);
        assert_eq!(v1.is_opposite(&v1, 1e-7).unwrap(), false);
        assert_eq!(v1.is_opposite(&v3, 1e-7).unwrap(), false);
    }

    #[test]
    fn test_is_parallel() {
        let v1 = Vector2d::from_coordinates(1.0, 0.0);
        let v2 = Vector2d::from_coordinates(10.0, 0.0);
        let v3 = Vector2d::from_coordinates(-1.0, 0.0);
        let v4 = Vector2d::from_coordinates(0.0, 1.0);
        assert_eq!(v1.is_parallel(&v2, 1e-7).unwrap(), true);
        assert_eq!(v1.is_parallel(&v3, 1e-7).unwrap(), true);
        assert_eq!(v1.is_parallel(&v4, 1e-7).unwrap(), false);
    }

    #[test]
    fn test_neg() {
        let v1 = Vector2d::from_coordinates(1.0, 2.0);
        let v2 = -v1;
        assert_eq!(v2.xy.x, -1.0);
        assert_eq!(v2.xy.y, -2.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vector2d::from_coordinates(1.0, 2.0);
        let v2 = Vector2d::from_coordinates(3.0, 4.0);
        let v3 = v1 + &v2;
        assert_eq!(v3.xy.x, 4.0);
        assert_eq!(v3.xy.y, 6.0);
        let v4 = v1 + 2.0;
        assert_eq!(v4.xy.x, 3.0);
        assert_eq!(v4.xy.y, 4.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vector2d::from_coordinates(1.0, 2.0);
        let v2 = Vector2d::from_coordinates(3.0, 4.0);
        assert_eq!(v1.cross(&v2), -2.0);
        assert_eq!(v1.cross_magnitude(&v2), 2.0);
        assert_eq!(v1.square_cross_magnitude(&v2), 4.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vector2d::from_coordinates(1.0, 2.0);
        let v2 = Vector2d::from_coordinates(3.0, 4.0);
        assert_eq!(v1.dot(&v2), 11.0);
        assert_eq!(v1 * &v2, 11.0);
    }

    #[test]
    fn test_normalize() {
        let mut v1 = Vector2d::from_coordinates(3.0, 4.0);
        v1.normalize();
        assert_eq!(v1.xy.x, 0.6);
        assert_eq!(v1.xy.y, 0.8);

        let v2 = Vector2d::from_coordinates(3.0, 4.0);
        let v3 = v2.normalize_new();
        assert_eq!(v3.xy.x, 0.6);
        assert_eq!(v3.xy.y, 0.8);
    }

    #[test]
    fn test_reverse() {
        let mut v1 = Vector2d::from_coordinates(3.0, 4.0);
        v1.reverse();
        assert_eq!(v1.xy.x, -3.0);
        assert_eq!(v1.xy.y, -4.0);

        let v2 = Vector2d::from_coordinates(3.0, 4.0);
        let v3 = v2.reverse_new();
        assert_eq!(v3.xy.x, -3.0);
        assert_eq!(v3.xy.y, -4.0);
    }

    #[test]
    fn test_mirror() {
        let mut v1 = Vector2d::from_coordinates(1.0, -1.0);
        let v2 = Vector2d::from_coordinates(0.0, 1.0);
        v1.mirror_by_vector2d(&v2);
        assert_eq!(v1.x(), 0.0);
        assert_eq!(v1.y(), 1.0);

        let mut v1 = Vector2d::from_coordinates(10.0, 0.0);
        let v2 = Vector2d::from_coordinates(1.0, 1.0);
        v1.mirror_by_vector2d(&v2);
        println!("v1: {:?}", v1);
        assert_eq!((v1.x()-1.0).abs() < 1e-10, true);
        assert_eq!((v1.y()-1.0).abs() < 1e-10, true);
    }
}
