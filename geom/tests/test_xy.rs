use geom::XY;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p1 = XY::<f64>::new();
        assert_eq!(p1.x, 0.0);
        assert_eq!(p1.y, 0.0);
        println!("p1: {}", p1);
    }

    #[test]
    fn test_from_coords() {
        let p2 = XY::from_coords(1.0, 2.0);
        assert_eq!(p2.x, 1.0);
        assert_eq!(p2.y, 2.0);
    }

    #[test]
    fn test_coords() {
        let mut p3 = XY::from_coords(3.0, 4.0);
        assert_eq!(p3.get_coords(), (3.0, 4.0));

        p3.set_coords(10.0, 20.0);
        assert_eq!(p3.get_coords(), (10.0, 20.0));
    }

    #[test]
    fn test_x_y() {
        let mut p = XY::from_coords(5.0, 6.0);
        assert_eq!(p.get_x(), 5.0);
        assert_eq!(p.get_y(), 6.0);

        p.set_x(7.0);
        p.set_y(8.0);
        assert_eq!(p.get_x(), 7.0);
        assert_eq!(p.get_y(), 8.0);
    }

    #[test]
    fn test_is_equal() {
        let p1 = XY::from_coords(1.0, 2.0);
        let p2 = XY::from_coords(1.0, 2.0);
        let p3 = XY::from_coords(1.1, 2.1);

        assert!(p1.is_equal(&p2, 0.01));
        assert!(!p1.is_equal(&p3, 0.01));
        assert!(p1.is_equal(&p3, 0.2));
    }
}
