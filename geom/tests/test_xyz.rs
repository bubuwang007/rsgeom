use geom::XYZ;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p1 = XYZ::<f64>::new();
        assert_eq!(p1.x, 0.0);
        assert_eq!(p1.y, 0.0);
        assert_eq!(p1.z, 0.0);
        println!("p1: {}", p1);
    }

    #[test]
    fn test_from_coords() {
        let p2 = XYZ::from_coords(1.0, 2.0, 3.0);
        assert_eq!(p2.x, 1.0);
        assert_eq!(p2.y, 2.0);
        assert_eq!(p2.z, 3.0);
    }

    #[test]
    fn test_coords() {
        let mut p3 = XYZ::from_coords(3.0, 4.0, 5.0);
        assert_eq!(p3.get_coords(), (3.0, 4.0, 5.0));

        p3.set_coords(10.0, 20.0, 30.0);
        assert_eq!(p3.get_coords(), (10.0, 20.0, 30.0));
    }

    #[test]
    fn test_x_y_z() {
        let mut p = XYZ::from_coords(5.0, 6.0, 7.0);
        assert_eq!(p.get_x(), 5.0);
        assert_eq!(p.get_y(), 6.0);
        assert_eq!(p.get_z(), 7.0);

        p.set_x(8.0);
        p.set_y(9.0);
        p.set_z(10.0);
        assert_eq!(p.get_x(), 8.0);
        assert_eq!(p.get_y(), 9.0);
        assert_eq!(p.get_z(), 10.0);
    }

    #[test]
    fn test_is_equal() {
        let p1 = XYZ::from_coords(1.0, 2.0, 3.0);
        let p2 = XYZ::from_coords(1.0, 2.0, 3.0);
        let p3 = XYZ::from_coords(1.1, 2.1, 3.1);

        assert!(p1.is_equal(&p2, 0.01));
        assert!(!p1.is_equal(&p3, 0.01));
        assert!(p1.is_equal(&p3, 0.2));
    }
}
