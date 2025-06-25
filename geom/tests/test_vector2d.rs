use geom::Vector2d;
use geom::XY;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v1 = Vector2d::<f64>::new();
        assert_eq!(v1.xy.x, 0.0);
        assert_eq!(v1.xy.y, 0.0);
        println!("v1: {}", v1);
    }

    #[test]
    fn test_from_xy() {
        let xy = XY::from_coords(1.0, 2.0);
        let v2 = Vector2d::from_xy(xy);
        assert_eq!(v2.xy.x, 1.0);
        assert_eq!(v2.xy.y, 2.0);
    }

    #[test]
    fn test_from_coords() {
        let v3 = Vector2d::from_coords(3.0, 4.0);
        assert_eq!(v3.xy.x, 3.0);
        assert_eq!(v3.xy.y, 4.0);
    }

    #[test]
    fn test_coords() {
        let mut v4 = Vector2d::from_coords(5.0, 6.0);
        assert_eq!(v4.xy.get_coords(), (5.0, 6.0));

        v4.set_coords(10.0, 20.0);
        assert_eq!(v4.xy.get_coords(), (10.0, 20.0));
    }

    #[test]
    fn test_x_y() {
        let mut v5 = Vector2d::from_coords(7.0, 8.0);
        assert_eq!(v5.xy.get_x(), 7.0);
        assert_eq!(v5.xy.get_y(), 8.0);

        v5.set_x(9.0);
        v5.set_y(10.0);
        assert_eq!(v5.xy.get_x(), 9.0);
        assert_eq!(v5.xy.get_y(), 10.0);
    }

    #[test]
    fn test_xy() {
        let mut v = Vector2d::from_xy(XY::from_coords(11.0, 12.0));
        assert_eq!(v.xy.get_coords(), (11.0, 12.0));
        v.set_xy(XY::from_coords(13.0, 14.0));
        assert_eq!(v.xy.get_coords(), (13.0, 14.0));
    }

    #[test]
    fn test_display() {
        let v = Vector2d::from_coords(15.0, 16.0);
        assert_eq!(format!("{}", v), "Vector2d(15, 16)");
    }

    #[test]
    fn test_is_equal() {
        let v1 = Vector2d::from_coords(1.0, 2.0);
        let v2 = Vector2d::from_coords(1.0, 2.0);
        let v3 = Vector2d::from_coords(1.1, 2.1);

        assert!(v1.xy.is_equal(&v2.xy, 1e-6));
        assert!(!v1.xy.is_equal(&v3.xy, 1e-6));
    }
    

}