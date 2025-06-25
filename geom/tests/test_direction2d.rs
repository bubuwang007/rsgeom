use geom::Direction2d;
use geom::XY;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let d1 = Direction2d::<f64>::new();
        assert_eq!(d1.xy.x, 0.0);
        assert_eq!(d1.xy.y, 0.0);
        println!("d1: {}", d1);
    }

    #[test]
    fn test_from_xy() {
        let xy = XY::from_coords(1.0, 2.0);
        let d2 = Direction2d::from_xy(xy);
        assert_eq!(d2.xy.x, 1.0);
        assert_eq!(d2.xy.y, 2.0);
    }

    #[test]
    fn test_from_coords() {
        let d3 = Direction2d::from_coords(3.0, 4.0);
        assert_eq!(d3.xy.x, 3.0);
        assert_eq!(d3.xy.y, 4.0);
    }

    #[test]
    fn test_coords() {
        let mut d4 = Direction2d::from_coords(5.0, 6.0);
        assert_eq!(d4.xy.get_coords(), (5.0, 6.0));

        d4.set_coords(10.0, 20.0);
        assert_eq!(d4.xy.get_coords(), (10.0, 20.0));
    }

    #[test]
    fn test_x_y() {
        let mut d5 = Direction2d::from_coords(7.0, 8.0);
        assert_eq!(d5.xy.get_x(), 7.0);
        assert_eq!(d5.xy.get_y(), 8.0);

        d5.set_x(9.0);
        d5.set_y(10.0);
        assert_eq!(d5.xy.get_x(), 9.0);
        assert_eq!(d5.xy.get_y(), 10.0);
    }

    #[test]
    fn test_xy() {
        let mut d = Direction2d::from_xy(XY::from_coords(11.0, 12.0));
        assert_eq!(d.xy.get_coords(), (11.0, 12.0));
        d.set_xy(XY::from_coords(13.0, 14.0));
        assert_eq!(d.xy.get_coords(), (13.0, 14.0));
    }

    #[test]
    fn test_display() {
        let d = Direction2d::from_coords(15.0, 16.0);
        assert_eq!(format!("{}", d), "Direction2d(15, 16)");
    }

    #[test]
    fn test_is_equal() {
        let d1 = Direction2d::from_coords(1.0, 2.0);
        let d2 = Direction2d::from_coords(1.0, 2.0);
        let d3 = Direction2d::from_coords(1.1, 2.1);

        assert!(d1.is_equal(&d2, 0.01));
        assert!(!d1.is_equal(&d3, 0.01));
    }
}