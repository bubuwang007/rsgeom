use geom::Direction2d;
use geom::XY;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let d1 = Direction2d::<f64>::new();
        assert_eq!(d1.xy.x, 1.0);
        assert_eq!(d1.xy.y, 0.0);
        println!("d1: {}", d1);
    }

    #[test]
    fn test_from_xy() {
        let xy = XY::from_coords(1.0, 2.0);
        let d2 = Direction2d::from_xy(xy);
        assert_eq!(d2.xy.x, 0.4472135954999579);
        assert_eq!(d2.xy.y, 0.8944271909999159);
    }

    #[test]
    fn test_from_coords() {
        let d3 = Direction2d::from_coords(3.0, 4.0);
        assert_eq!(d3.xy.x, 0.6);
        assert_eq!(d3.xy.y, 0.8);
    }

    #[test]
    fn test_coords() {
        let mut d4 = Direction2d::from_coords(5.0, 6.0);
        assert_eq!(d4.xy.get_coords(), (0.6401843996644799, 0.7682212795973759));

        d4.set_coords(3.0, 4.0);
        assert_eq!(d4.xy.get_coords(), (0.6, 0.8));
    }

    #[test]
    fn test_x_y() {
        let mut d5 = Direction2d::from_coords(6.0, 8.0);
        assert_eq!(d5.get_x(), 0.6);
        assert_eq!(d5.get_y(), 0.8);

        d5.set_coords(3.0, 4.0);
        assert_eq!(d5.get_x(), 0.6);
        assert_eq!(d5.get_y(), 0.8);
    }

    #[test]
    fn test_xy() {
        let mut d = Direction2d::from_xy(XY::from_coords(3.0, 4.0));
        assert_eq!(d.xy.get_coords(), (0.6, 0.8));
        d.set_xy(XY::from_coords(6.0, 8.0));
        assert_eq!(d.xy.get_coords(), (0.6, 0.8));
    }

    #[test]
    fn test_display() {
        let d = Direction2d::from_coords(3.0, 4.0);
        assert_eq!(format!("{}", d), "Direction2d(0.6, 0.8)");
    }

    #[test]
    fn test_is_equal() {
        let d1 = Direction2d::from_coords(1.0, 2.0);
        let d2 = Direction2d::from_coords(1.0, 2.0);
        let d3 = Direction2d::from_coords(1.1, 2.1);

        assert!(d1.is_equal(&d2, 0.01));
        assert!(!d1.is_equal(&d3, 0.01));
    }

    #[test]
    fn test_angle() {
        let d1 = Direction2d::from_coords(1.0, 0.0);
        let d2 = Direction2d::from_coords(0.0, 1.0);
        let d3 = Direction2d::from_coords(1.0, 1.0);
        let angle = d1.angle(&d2);
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-10, "Angle should be π/2 radians");
        let angle2 = d1.angle(&d3);
        assert!((angle2 - std::f64::consts::FRAC_PI_4).abs() < 1e-10, "Angle should be π/4 radians");
    }

    #[test]
    fn test_is_orthogonal() {
        let d1 = Direction2d::from_coords(1.0, 0.0);
        let d2 = Direction2d::from_coords(0.0, 1.0);
        let d3 = Direction2d::from_coords(1.0, 1.0);
        assert!(d1.is_orthogonal(&d2, 1e-10));
        assert!(!d1.is_orthogonal(&d3, 1e-10));
    }

    #[test]
    fn test_is_opposite() {
        let d1 = Direction2d::from_coords(1.0, 0.0);
        let d2 = Direction2d::from_coords(-1.0, 0.0);
        let d3 = Direction2d::from_coords(0.0, 1.0);
        assert!(d1.is_opposite(&d2, 1e-10));
        assert!(!d1.is_opposite(&d3, 1e-10));
    }

    #[test]
    fn test_is_parallel() {
        let d1 = Direction2d::from_coords(1.0, 0.0);
        let d2 = Direction2d::from_coords(2.0, 0.0);
        let d3 = Direction2d::from_coords(0.0, 1.0);
        let d4 = Direction2d::from_coords(-1.0, 0.0);
        assert!(d1.is_parallel(&d2, 1e-10));
        assert!(!d1.is_parallel(&d3, 1e-10));
        assert!(d1.is_parallel(&d4, 1e-10));
    }
}
