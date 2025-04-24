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

}
