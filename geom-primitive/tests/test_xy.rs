use geom_primitive::xy::XY;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let p1 = XY::new();
        assert_eq!(p1.x, 0.0);
        assert_eq!(p1.y, 0.0);
        let p2 = XY::from_coordinates(1.0, 2.0);
        assert_eq!(p2.x, 1.0);
        assert_eq!(p2.y, 2.0);

        let mut p3 = XY::new();
        p3.x = 3.0;
        p3.y = 4.0;
        assert_eq!(p3.x, 3.0);
        assert_eq!(p3.y, 4.0);
    }

    #[test]
    fn test_index() {
        let p = XY::from_coordinates(1.0, 2.0);
        assert_eq!(p[0], 1.0);
        assert_eq!(p[1], 2.0);

        let mut p_mut = p;
        p_mut[0] = 3.0;
        p_mut[1] = 4.0;
        assert_eq!(p_mut[0], 3.0);
        assert_eq!(p_mut[1], 4.0);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_index_out_of_bounds() {
        let p = XY::from_coordinates(1.0, 2.0);
        let _ = p[2]; // This should panic
    }

    #[test]
    fn test_change_coord() {
        let mut p = XY::from_coordinates(1.0, 2.0);
        let x_coord = p.change_coord(0);
        *x_coord = 3.0;
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 2.0);
    }

    #[test]
    fn test_coord() {
        let mut p = XY::new();
        p.coord(5.0, 6.0);
        assert_eq!(p.x, 5.0);
        assert_eq!(p.y, 6.0);
    }

    #[test]
    fn test_modulus() {
        let p = XY::from_coordinates(3.0, 4.0);
        assert_eq!(p.modulus(), 5.0);
    }

    #[test]
    fn test_square_modulus() {
        let p = XY::from_coordinates(3.0, 4.0);
        assert_eq!(p.square_modulus(), 25.0);
    }

    #[test]
    fn test_is_equal() {
        let p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = XY::from_coordinates(1.0, 2.0);
        assert!(p1.is_equal(&p2, 0.01));

        // let p3 = XY::from_coordinates(1.0, 2.01);
        // assert!(!p1.is_equal(&p3, 0.011));
    }


}