use geom_primitive::xyz::XYZ;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let p1 = XYZ::new();
        assert_eq!(p1.x, 0.0);
        assert_eq!(p1.y, 0.0);
        assert_eq!(p1.z, 0.0);
        let p2 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        assert_eq!(p2.x, 1.0);
        assert_eq!(p2.y, 2.0);
        assert_eq!(p2.z, 3.0);

        let mut p3 = XYZ::new();
        p3.x = 4.0;
        p3.y = 5.0;
        p3.z = 6.0;
        assert_eq!(p3.x, 4.0);
        assert_eq!(p3.y, 5.0);
        assert_eq!(p3.z, 6.0);
    }

    #[test]
    fn test_coord() {
        let mut p = XYZ::new();
        p.set_coord(7.0, 8.0, 9.0);
        assert_eq!(p.coord(), (7.0, 8.0, 9.0));
    }

    #[test]
    fn test_modulus() {
        let p = XYZ::from_coordinates(3.0, 4.0, 5.0);
        assert_eq!(p.modulus(), ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt());
    }

    #[test]
    fn test_square_modulus() {
        let p = XYZ::from_coordinates(3.0, 4.0, 5.0);
        assert_eq!(p.square_modulus(), 50.0);
    }

    #[test]
    fn test_is_equal() {
        let p1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let p2 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let p3 = XYZ::from_coordinates(1.0, 2.0, 4.0);
        assert!(p1.is_equal(&p2, 1e-6));
        assert!(!p1.is_equal(&p3, 1e-6));
    }

    #[test]
    fn test_index() {
        let p = XYZ::from_coordinates(1.0, 2.0, 3.0);
        assert_eq!(p[0], 1.0);
        assert_eq!(p[1], 2.0);
        assert_eq!(p[2], 3.0);

        let mut p_mut = p;
        p_mut[0] = 4.0;
        p_mut[1] = 5.0;
        p_mut[2] = 6.0;
        assert_eq!(p_mut[0], 4.0);
        assert_eq!(p_mut[1], 5.0);
        assert_eq!(p_mut[2], 6.0);
    }

    #[test]
    fn test_to_string() {
        let p = XYZ::from_coordinates(1.0, 2.0, 3.0);
        assert_eq!(p.to_string(), "XYZ(1, 2, 3)");
    }

}
