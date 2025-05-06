use geom_primitive::xyz::XYZ;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let p1: XYZ<f64> = XYZ::new();
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
        assert_eq!(
            p.modulus(),
            ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
        );
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

    #[test]
    fn test_add() {
        let mut p1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let p2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let f = 2.0;
        p1 += &p2;
        assert_eq!(p1.x, 5.0);
        assert_eq!(p1.y, 7.0);
        assert_eq!(p1.z, 9.0);
        p1 += f;
        assert_eq!(p1.x, 7.0);
        assert_eq!(p1.y, 9.0);
        assert_eq!(p1.z, 11.0);

        let p3 = &p1 + &p2;
        assert_eq!(p3.x, 11.0);
        assert_eq!(p3.y, 14.0);
        assert_eq!(p3.z, 17.0);

        let p4 = p1 + f;
        assert_eq!(p4.x, 9.0);
        assert_eq!(p4.y, 11.0);
        assert_eq!(p4.z, 13.0);
    }

    #[test]
    fn test_cross() {
        let mut p1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let p2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let p3 = p1.cross_new(&p2);
        assert_eq!(p3.x, -3.0);
        assert_eq!(p3.y, 6.0);
        assert_eq!(p3.z, -3.0);

        p1.cross(&p2);
        assert_eq!(p1.x, -3.0);
        assert_eq!(p1.y, 6.0);
        assert_eq!(p1.z, -3.0);
    }

    #[test]
    fn test_cross_abs() {
        let p1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let p2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let abs = p1.cross_abs(&p2);
        assert_eq!(abs, 7.3484692283495345);
    }

    #[test]
    fn test_square_cross() {
        let p1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let p2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let square = p1.square_cross(&p2);
        assert_eq!(square, 54.0);
    }

    #[test]
    fn test_cross_cross() {
        let mut p1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let p2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let p3 = XYZ::from_coordinates(7.0, 8.0, 9.0);

        let p4 = p1.cross_cross_new(&p2, &p3);
        assert_eq!(p4.x, -24.0);
        assert_eq!(p4.y, -6.0);
        assert_eq!(p4.z, 12.0);

        p1.cross_cross(&p2, &p3);
        assert_eq!(p1.x, -24.0);
        assert_eq!(p1.y, -6.0);
        assert_eq!(p1.z, 12.0);
    }

    #[test]
    fn test_div() {
        let mut p1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let p2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let f = 2.0;
        p1 /= &p2;
        assert_eq!(p1.x, 0.25);
        assert_eq!(p1.y, 0.4);
        assert_eq!(p1.z, 0.5);

        p1 /= f;
        assert_eq!(p1.x, 0.125);
        assert_eq!(p1.y, 0.2);
        assert_eq!(p1.z, 0.25);

        let p3 = &p1 / &p2;
        assert_eq!(p3.x, 0.03125);
        assert_eq!(p3.y, 0.04);
        assert_eq!(p3.z, 0.041666666666666664);

        let p4 = &p1 / f;
        assert_eq!(p4.x, 0.0625);
        assert_eq!(p4.y, 0.1);
        assert_eq!(p4.z, 0.125);
    }

    #[test]
    fn test_dot() {
        let p1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let p2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let dot = p1.dot(&p2);
        assert_eq!(dot, 32.0);
    }

    #[test]
    fn test_dot_cross() {
        let p1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let p2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let p3 = XYZ::from_coordinates(7.0, 7.0, 9.0);
        let dot_cross = p1.dot_cross(&p2, &p3);
        assert_eq!(dot_cross, -6.0);
    }

    #[test]
    fn test_mul() {
        let mut p1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let p2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let f = 2.0;

        p1 *= &p2;
        assert_eq!(p1.x, 4.0);
        assert_eq!(p1.y, 10.0);
        assert_eq!(p1.z, 18.0);

        p1 *= f;
        assert_eq!(p1.x, 8.0);
        assert_eq!(p1.y, 20.0);
        assert_eq!(p1.z, 36.0);

        let p3 = &p1 * &p2;
        assert_eq!(p3.x, 32.0);
        assert_eq!(p3.y, 100.0);
        assert_eq!(p3.z, 216.0);

        let p4 = &p1 * f;
        assert_eq!(p4.x, 16.0);
        assert_eq!(p4.y, 40.0);
        assert_eq!(p4.z, 72.0);

        let p5: XYZ = f * &p1;
        assert_eq!(p5.x, 16.0);
        assert_eq!(p5.y, 40.0);
        assert_eq!(p5.z, 72.0);
    }

    #[test]
    fn test_normalize() {
        let mut p = XYZ::from_coordinates(3.0, 4.0, 5.0);
        p.normalize();
        assert_eq!(p.x, 3.0 / 7.0710678118654755);
        assert_eq!(p.y, 4.0 / 7.0710678118654755);
        assert_eq!(p.z, 5.0 / 7.0710678118654755);
    }

    #[test]
    fn test_reverse() {
        let mut p = XYZ::from_coordinates(3.0, 4.0, 5.0);
        p.reverse();
        assert_eq!(p.x, -3.0);
        assert_eq!(p.y, -4.0);
        assert_eq!(p.z, -5.0);

        let p2 = p.reverse_new();
        assert_eq!(p2.x, 3.0);
        assert_eq!(p2.y, 4.0);
        assert_eq!(p2.z, 5.0);

        let p3 = -&p;
        assert_eq!(p3.x, 3.0);
        assert_eq!(p3.y, 4.0);
        assert_eq!(p3.z, 5.0);
    }

    #[test]
    fn test_sub() {
        let mut p1 = XYZ::from_coordinates(4.0, 8.0, 12.0);
        let p2 = XYZ::from_coordinates(2.0, 4.0, 6.0);
        p1 -= &p2;
        assert_eq!(p1.x, 2.0);
        assert_eq!(p1.y, 4.0);
        assert_eq!(p1.z, 6.0);

        let p3 = &p1 - &p2;
        assert_eq!(p3.x, 0.0);
        assert_eq!(p3.y, 0.0);
        assert_eq!(p3.z, 0.0);

        let p4 = &p1 - 2.0;
        assert_eq!(p4.x, 0.0);
        assert_eq!(p4.y, 2.0);
        assert_eq!(p4.z, 4.0);
    }

    #[test]
    fn test_set_linear_form_2() {
        let mut p = XYZ::new();
        let xy1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let xy2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        p.set_linear_form_2(2.0, &xy1, 3.0, &xy2);
        assert_eq!(p.x, 14.0);
        assert_eq!(p.y, 19.0);
        assert_eq!(p.z, 24.0);
    }

    #[test]
    fn test_set_linear_form_2a() {
        let mut p = XYZ::new();
        let xy1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let xy2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        p.set_linear_form_2a(2.0, &xy1, &xy2);
        assert_eq!(p.x, 6.0);
        assert_eq!(p.y, 9.0);
        assert_eq!(p.z, 12.0);
    }

    #[test]
    fn test_set_linear_form_2b() {
        let mut p = XYZ::new();
        let xy1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let xy2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        p.set_linear_form_2b(&xy1, &xy2);
        assert_eq!(p.x, 5.0);
        assert_eq!(p.y, 7.0);
        assert_eq!(p.z, 9.0);
    }

    #[test]
    fn test_set_linear_form_3() {
        let mut p = XYZ::new();
        let xy1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let xy2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let xy3 = XYZ::from_coordinates(7.0, 8.0, 9.0);
        p.set_linear_form_3(2.0, &xy1, 3.0, &xy2, &xy3);
        assert_eq!(p.x, 21.0);
        assert_eq!(p.y, 27.0);
        assert_eq!(p.z, 33.0);
    }

    #[test]
    fn test_set_linear_form_3a() {
        let mut p = XYZ::new();
        let xy1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let xy2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let xy3 = XYZ::from_coordinates(7.0, 8.0, 9.0);
        p.set_linear_form_3a(2.0, &xy1, 3.0, &xy2, 4.0, &xy3);
        assert_eq!(p.x, 42.0);
        assert_eq!(p.y, 51.0);
        assert_eq!(p.z, 60.0);
    }

    #[test]
    fn test_set_linear_form_4() {
        let mut p = XYZ::new();
        let xy1 = XYZ::from_coordinates(1.0, 2.0, 3.0);
        let xy2 = XYZ::from_coordinates(4.0, 5.0, 6.0);
        let xy3 = XYZ::from_coordinates(7.0, 8.0, 9.0);
        let xy4 = XYZ::from_coordinates(10.0, 11.0, 12.0);
        p.set_linear_form_4(2.0, &xy1, 3.0, &xy2, 4.0, &xy3, &xy4);
        assert_eq!(p.x, 52.0);
        assert_eq!(p.y, 62.0);
        assert_eq!(p.z, 72.0);
    }
}
