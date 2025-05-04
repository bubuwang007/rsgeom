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
    fn test_to_string() {
        let p = XY::from_coordinates(1.0, 2.0);
        assert_eq!(p.to_string(), "XY(1, 2)");
    }

    #[test]
    fn test_coord() {
        let mut p = XY::new();
        p.set_coord(5.0, 6.0);
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

        let p3 = XY::from_coordinates(1.0, 2.01);
        assert!(p1.is_equal(&p3, 0.01));
    }

    #[test]
    fn test_add_assign() {
        let mut p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = XY::from_coordinates(3.0, 4.0);
        p1 += &p2;
        assert_eq!(p1.x, 4.0);
        assert_eq!(p1.y, 6.0);
    }

    #[test]
    fn test_add() {
        let p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = XY::from_coordinates(3.0, 4.0);
        let p3 = &p1 + &p2;
        assert_eq!(p3.x, 4.0);
        assert_eq!(p3.y, 6.0);
    }

    #[test]
    fn test_add_assign_f64() {
        let mut p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = 3.0;
        p1 += p2;
        assert_eq!(p1.x, 4.0);
        assert_eq!(p1.y, 5.0);
    }

    #[test]
    fn test_add_f64() {
        let p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = 3.0;
        let p3 = p1 + p2;
        assert_eq!(p3.x, 4.0);
        assert_eq!(p3.y, 5.0);
    }

    #[test]
    fn test_cross() {
        let p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = XY::from_coordinates(3.0, 4.0);
        assert_eq!(p1.cross(&p2), -2.0);
    }

    #[test]
    fn test_cross_magnitude() {
        let p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = XY::from_coordinates(3.0, 4.0);
        assert_eq!(p1.cross_abs(&p2), 2.0);
    }

    #[test]
    fn test_cross_magnitude_squared() {
        let p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = XY::from_coordinates(3.0, 4.0);
        assert_eq!(p1.square_cross_abs(&p2), 4.0);
    }

    #[test]
    fn test_div_assign() {
        let mut p1 = XY::from_coordinates(4.0, 8.0);
        let p2 = XY::from_coordinates(2.0, 4.0);
        p1 /= &p2;
        assert_eq!(p1.x, 2.0);
        assert_eq!(p1.y, 2.0);
    }

    #[test]
    fn test_div() {
        let p1 = XY::from_coordinates(4.0, 8.0);
        let p2 = XY::from_coordinates(2.0, 4.0);
        let p3 = &p1 / &p2;
        assert_eq!(p3.x, 2.0);
        assert_eq!(p3.y, 2.0);
    }

    #[test]
    fn test_div_assign_f64() {
        let mut p1 = XY::from_coordinates(4.0, 8.0);
        let p2 = 2.0;
        p1 /= p2;
        assert_eq!(p1.x, 2.0);
        assert_eq!(p1.y, 4.0);
    }

    #[test]
    fn test_div_f64() {
        let p1 = XY::from_coordinates(4.0, 8.0);
        let p2 = 2.0;
        let p3 = &p1 / p2;
        assert_eq!(p3.x, 2.0);
        assert_eq!(p3.y, 4.0);
    }

    #[test]
    fn test_dot() {
        let p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = XY::from_coordinates(3.0, 4.0);
        assert_eq!(p1.dot(&p2), 11.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = XY::from_coordinates(3.0, 4.0);
        p1 *= &p2;
        assert_eq!(p1.x, 3.0);
        assert_eq!(p1.y, 8.0);
    }

    #[test]
    fn test_mul() {
        let p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = XY::from_coordinates(3.0, 4.0);
        let p3 = &p1 * &p2;
        assert_eq!(p3.x, 3.0);
        assert_eq!(p3.y, 8.0);
    }

    #[test]
    fn test_mul_assign_f64() {
        let mut p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = 3.0;
        p1 *= p2;
        assert_eq!(p1.x, 3.0);
        assert_eq!(p1.y, 6.0);
    }

    #[test]
    fn test_f64_mul_xy() {
        let p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = 3.0;
        let p3 = p2 * &p1;
        assert_eq!(p3.x, 3.0);
        assert_eq!(p3.y, 6.0);
    }

    #[test]
    fn test_mul_f64() {
        let p1 = XY::from_coordinates(1.0, 2.0);
        let p2 = 3.0;
        let p3 = &p1 * p2;
        assert_eq!(p3.x, 3.0);
        assert_eq!(p3.y, 6.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut p1 = XY::from_coordinates(4.0, 8.0);
        let p2 = XY::from_coordinates(2.0, 4.0);
        p1 -= &p2;
        assert_eq!(p1.x, 2.0);
        assert_eq!(p1.y, 4.0);
    }

    #[test]
    fn test_sub() {
        let p1 = XY::from_coordinates(4.0, 8.0);
        let p2 = XY::from_coordinates(2.0, 4.0);
        let p3 = &p1 - &p2;
        assert_eq!(p3.x, 2.0);
        assert_eq!(p3.y, 4.0);
    }

    #[test]
    fn test_sub_assign_f64() {
        let mut p1 = XY::from_coordinates(4.0, 8.0);
        let p2 = 2.0;
        p1 -= p2;
        assert_eq!(p1.x, 2.0);
        assert_eq!(p1.y, 6.0);
    }

    #[test]
    fn test_sub_f64() {
        let p1 = XY::from_coordinates(4.0, 8.0);
        let p2 = 2.0;
        let p3 = &p1 - p2;
        assert_eq!(p3.x, 2.0);
        assert_eq!(p3.y, 6.0);
    }

    #[test]
    fn test_sub_xy() {
        let p1 = XY::from_coordinates(4.0, 8.0);
        let p2 = XY::from_coordinates(2.0, 4.0);
        let p3 = &p1 - &p2;
        assert_eq!(p3.x, 2.0);
        assert_eq!(p3.y, 4.0);
    }

    #[test]
    fn test_normalize() {
        let mut p = XY::from_coordinates(3.0, 4.0);
        p.normalize();
        assert_eq!(p.x, 0.6);
        assert_eq!(p.y, 0.8);
    }

    #[test]
    fn test_normalize_new() {
        let p = XY::from_coordinates(3.0, 4.0).normalize_new();
        assert_eq!(p.x, 0.6);
        assert_eq!(p.y, 0.8);
    }

    #[test]
    fn test_reverse() {
        let mut p = XY::from_coordinates(3.0, 4.0);
        p.reverse();
        assert_eq!(p.x, -3.0);
        assert_eq!(p.y, -4.0);
    }

    #[test]
    fn test_reverse_new() {
        let p = XY::from_coordinates(3.0, 4.0).reverse_new();
        assert_eq!(p.x, -3.0);
        assert_eq!(p.y, -4.0);
    }

    #[test]
    fn test_neg() {
        let p = -&XY::from_coordinates(3.0, 4.0);
        assert_eq!(p.x, -3.0);
        assert_eq!(p.y, -4.0);
    }

    #[test]
    fn test_set_linear_form_2() {
        let mut p = XY::new();
        let xy1 = XY::from_coordinates(1.0, 2.0);
        let xy2 = XY::from_coordinates(3.0, 4.0);
        p.set_linear_form_2(2.0, &xy1, 3.0, &xy2);
        assert_eq!(p.x, 11.0);
        assert_eq!(p.y, 16.0);
    }

    #[test]
    fn test_set_linear_form_3() {
        let mut p = XY::new();
        let xy1 = XY::from_coordinates(1.0, 2.0);
        let xy2 = XY::from_coordinates(3.0, 4.0);
        let xy3 = XY::from_coordinates(5.0, 6.0);
        p.set_linear_form_3(2.0, &xy1, 3.0, &xy2, &xy3);
        assert_eq!(p.x, 16.0);
        assert_eq!(p.y, 22.0);
    }

    #[test]
    fn test_set_linear_form_2a() {
        let mut p = XY::new();
        let xy1 = XY::from_coordinates(1.0, 2.0);
        let xy2 = XY::from_coordinates(3.0, 4.0);
        p.set_linear_form_2a(2.0, &xy1, &xy2);
        assert_eq!(p.x, 5.0);
        assert_eq!(p.y, 8.0);
    }

    #[test]
    fn test_set_linear_form_2b() {
        let mut p = XY::new();
        let xy1 = XY::from_coordinates(1.0, 2.0);
        let xy2 = XY::from_coordinates(3.0, 4.0);
        p.set_linear_form_2b(&xy1, &xy2);
        assert_eq!(p.x, 4.0);
        assert_eq!(p.y, 6.0);
    }
}
