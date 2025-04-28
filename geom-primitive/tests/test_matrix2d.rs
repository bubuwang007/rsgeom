use geom_primitive::matrix2d::Matrix2d;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let m1 = Matrix2d::new();
        assert_eq!(m1[0][0], 0.0);
        assert_eq!(m1[0][1], 0.0);
        assert_eq!(m1[1][0], 0.0);
        assert_eq!(m1[1][1], 0.0);

        let m2 = Matrix2d::from_coordinates(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m2[0][0], 1.0);
        assert_eq!(m2[0][1], 2.0);
        assert_eq!(m2[1][0], 3.0);
        assert_eq!(m2[1][1], 4.0);

        let mut m3 = Matrix2d::new();
        m3.set_col(0, [5.0, 6.0]);
        m3.set_row(1, [7.0, 8.0]);
        assert_eq!(m3.col(0), [5.0, 7.0]);
        assert_eq!(m3.row(1), [7.0, 8.0]);

        let m4 = Matrix2d::from_array([[1.0, 2.0], [3.0, 4.0]]);
        assert_eq!(m4[0][0], 1.0);
        assert_eq!(m4[0][1], 2.0);
        assert_eq!(m4[1][0], 3.0);
        assert_eq!(m4[1][1], 4.0);
    }

    #[test]
    fn test_xy() {
        let m = Matrix2d::from_coordinates(1.0, 2.0, 3.0, 4.0);
        let xy1 = m.col_xy(0);
        let xy2 = m.row_xy(1);
        assert_eq!(xy1.x, 1.0);
        assert_eq!(xy1.y, 3.0);
        assert_eq!(xy2.x, 3.0);
        assert_eq!(xy2.y, 4.0);
    }

    #[test]
    fn test_diagonal() {
        let mut m = Matrix2d::new();
        m.set_diagonal([5.0, 6.0]);
        assert_eq!(m.diagonal(), [5.0, 6.0]);
        assert_eq!(m[0][0], 5.0);
        assert_eq!(m[1][1], 6.0);
    }

    #[test]
    fn test_identity() {
        let mut m = Matrix2d::new();
        m.set_identity();
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[1][1], 1.0);
        assert_eq!(m[0][1], 0.0);
        assert_eq!(m[1][0], 0.0);
    }

    #[test]
    fn test_rotation() {
        let mut m = Matrix2d::new();
        m.set_rotation(45_f64.to_radians());
        assert!((m[0][0] - 0.7071).abs() < 0.0001);
        assert!((m[0][1] + 0.7071).abs() < 0.0001);
        assert!((m[1][0] - 0.7071).abs() < 0.0001);
        assert!((m[1][1] - 0.7071).abs() < 0.0001);
    }

    #[test]
    fn test_determinant() {
        let m = Matrix2d::from_coordinates(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m.determinant(), -2.0);
    }

    #[test]
    fn test_add_assign() {
        let mut m1 = Matrix2d::from_coordinates(1.0, 2.0, 3.0, 4.0);
        let m2 = Matrix2d::from_coordinates(5.0, 6.0, 7.0, 8.0);
        m1 += &m2;
        assert_eq!(m1[0][0], 6.0);
        assert_eq!(m1[0][1], 8.0);
        assert_eq!(m1[1][0], 10.0);
        assert_eq!(m1[1][1], 12.0);

        let m3 = &m1 + &m2;
        assert_eq!(m3[0][0], 11.0);
        assert_eq!(m3[0][1], 14.0);
        assert_eq!(m3[1][0], 17.0);
        assert_eq!(m3[1][1], 20.0);

        let m4 = &m1 + 2.0;
        assert_eq!(m4[0][0], 8.0);
        assert_eq!(m4[0][1], 10.0);
        assert_eq!(m4[1][0], 12.0);
        assert_eq!(m4[1][1], 14.0);
    }

    #[test]
    fn test_invert() {
        let mut m = Matrix2d::from_coordinates(1.0, 2.0, 3.0, 4.0);
        assert!(!m.is_singular());
        m.invert().unwrap();
        println!("{}", m.to_string());
        assert!((m[0][0] + 2.0).abs() < 0.0001);
        assert!((m[0][1] - 1.0).abs() < 0.0001);
        assert!((m[1][0] - 1.5).abs() < 0.0001);
        assert!((m[1][1] + 0.5).abs() < 0.0001);

        let mut m_singular = Matrix2d::from_coordinates(1.0, 2.0, 2.0, 4.0);
        assert!(m_singular.is_singular());
        assert!(m_singular.invert().is_err());
    }

    #[test]
    fn test_matmul(){
        let mut m1 = Matrix2d::from_coordinates(1.0, 2.0, 3.0, 4.0);
        let m2 = Matrix2d::from_coordinates(5.0, 6.0, 7.0, 8.0);
        m1.matmul(&m2);
        assert_eq!(m1[0][0], 19.0);
        assert_eq!(m1[0][1], 22.0);
        assert_eq!(m1[1][0], 43.0);
        assert_eq!(m1[1][1], 50.0);

        let m3 = m1.matmul_new(&m2);
        assert_eq!(m3[0][0], 249.0);
        assert_eq!(m3[0][1], 290.0);
        assert_eq!(m3[1][0], 565.0);
        assert_eq!(m3[1][1], 658.0);
    }

    #[test]
    fn test_left_matmul(){
        let mut m1 = Matrix2d::from_coordinates(1.0, 2.0, 3.0, 4.0);
        let m2 = Matrix2d::from_coordinates(5.0, 6.0, 7.0, 8.0);
        m1.left_matmul(&m2);
        assert_eq!(m1[0][0], 23.0);
        assert_eq!(m1[0][1], 34.0);
        assert_eq!(m1[1][0], 31.0);
        assert_eq!(m1[1][1], 46.0);

        let m3 = m1.left_matmul_new(&m2);
        assert_eq!(m3[0][0], 301.0);
        assert_eq!(m3[0][1], 446.0);
        assert_eq!(m3[1][0], 409.0);
        assert_eq!(m3[1][1], 606.0);
    }

    #[test]
    fn test_matpowi() {
        let mut m = Matrix2d::from_coordinates(1.0, 2.0, 3.0, 4.0);
        m.matpowi(2);
        assert_eq!(m[0][0], 7.0);
        assert_eq!(m[0][1], 10.0);
        assert_eq!(m[1][0], 15.0);
        assert_eq!(m[1][1], 22.0);

        let m2 = m.matpowi_new(2);
        assert_eq!(m2[0][0], 199.0);
        assert_eq!(m2[0][1], 290.0);
        assert_eq!(m2[1][0], 435.0);
        assert_eq!(m2[1][1], 634.0);
    }

    #[test]
    fn test_pow() {
        let mut m = Matrix2d::from_coordinates(1.0, 2.0, 3.0, 4.0);
        m.powi(2);
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 4.0);
        assert_eq!(m[1][0], 9.0);
        assert_eq!(m[1][1], 16.0);

        let mut m = Matrix2d::from_coordinates(1.0, 2.0, 3.0, 4.0);
        m.powf(2.5);
        assert!((m[0][0] - 1.0).abs() < 0.0001);
        assert!((m[0][1] - 5.656854).abs() < 0.0001);
        assert!((m[1][0] - 15.588457).abs() < 0.0001);
        assert!((m[1][1] - 32.0).abs() < 0.0001);
    }

    #[test]
    fn test_f64_mul() {
        let m = Matrix2d::from_coordinates(1.0, 2.0, 3.0, 4.0);
        let f = 2.0;
        let result = f * &m;
        assert_eq!(result[0][0], 2.0);
        assert_eq!(result[0][1], 4.0);
        assert_eq!(result[1][0], 6.0);
        assert_eq!(result[1][1], 8.0);
    }

}
