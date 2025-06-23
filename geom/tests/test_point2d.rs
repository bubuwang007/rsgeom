use geom::Point2d;
use geom::XY;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p1 = Point2d::<f64>::new();
        assert_eq!(p1.xy.x, 0.0);
        assert_eq!(p1.xy.y, 0.0);
        println!("p1: {}", p1);
    }

    #[test]
    fn test_from_xy() {
        let xy = XY::from_coords(1.0, 2.0);
        let p2 = Point2d::from_xy(xy);
        assert_eq!(p2.xy.x, 1.0);
        assert_eq!(p2.xy.y, 2.0);
    }

    #[test]
    fn test_from_coords() {
        let p3 = Point2d::from_coords(3.0, 4.0);
        assert_eq!(p3.xy.x, 3.0);
        assert_eq!(p3.xy.y, 4.0);
    }

    #[test]
    fn test_coords() {
        let mut p4 = Point2d::from_coords(5.0, 6.0);
        assert_eq!(p4.xy.get_coords(), (5.0, 6.0));

        p4.set_coords(10.0, 20.0);
        assert_eq!(p4.xy.get_coords(), (10.0, 20.0));
    }

    #[test]
    fn test_x_y() {
        let mut p5 = Point2d::from_coords(7.0, 8.0);
        assert_eq!(p5.xy.get_x(), 7.0);
        assert_eq!(p5.xy.get_y(), 8.0);

        p5.set_x(9.0);
        p5.set_y(10.0);
        assert_eq!(p5.xy.get_x(), 9.0);
        assert_eq!(p5.xy.get_y(), 10.0);
    }

    #[test]
    fn test_xy() {
        let mut p = Point2d::from_xy(XY::from_coords(11.0, 12.0));
        assert_eq!(p.xy.get_coords(), (11.0, 12.0));
        p.set_xy(XY::from_coords(13.0, 14.0));
        assert_eq!(p.xy.get_coords(), (13.0, 14.0));
    }

    #[test]
    fn test_is_equal() {
        let p1 = Point2d::from_coords(1.0, 2.0);
        let p2 = Point2d::from_coords(1.0, 2.0);
        let p3 = Point2d::from_coords(1.1, 2.1);

        assert!(p1.is_equal(&p2, 0.01));
        assert!(!p1.is_equal(&p3, 0.01));
        assert!(p1.is_equal(&p3, 0.2));
    }
}
