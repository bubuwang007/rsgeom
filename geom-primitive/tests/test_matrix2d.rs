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
}
