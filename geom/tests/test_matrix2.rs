use geom::Matrix2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m1 = Matrix2::<f64>::new();
        assert_eq!(m1.get_values(), [[0.0, 0.0], [0.0, 0.0]]);
        println!("m1: {}", m1);
    }

    #[test]
    fn test_from_array() {
        let m2 = Matrix2::from_array([[1.0, 2.0], [3.0, 4.0]]);
        assert_eq!(m2.get_values(), [[1.0, 2.0], [3.0, 4.0]]);
        println!("m2: {}", m2);
    }

    #[test]
    fn test_from_values() {
        let m3 = Matrix2::from_values(5.0, 6.0, 7.0, 8.0);
        assert_eq!(m3.get_values(), [[5.0, 6.0], [7.0, 8.0]]);
        println!("m3: {}", m3);
    }

    #[test]
    fn test_get_set_values() {
        let mut m4 = Matrix2::new();
        m4.set_values(9.0, 10.0, 11.0, 12.0);
        assert_eq!(m4.get_values(), [[9.0, 10.0], [11.0, 12.0]]);
        println!("m4: {}", m4);
    }

    #[test]
    fn test_get_set_col() {
        let mut m5 = Matrix2::from_values(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m5.get_col(0), [1.0, 3.0]);
        m5.set_col(0, [5.0, 6.0]);
        assert_eq!(m5.get_col(0), [5.0, 6.0]);
        println!("m5: {}", m5);
    }

    #[test]
    fn test_get_set_row() {
        let mut m6 = Matrix2::from_values(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m6.get_row(1), [3.0, 4.0]);
        m6.set_row(1, [5.0, 6.0]);
        assert_eq!(m6.get_row(1), [5.0, 6.0]);
        println!("m6: {}", m6);
    }

    #[test]
    fn test_get_set_diagonal() {
        let mut m7 = Matrix2::from_values(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m7.get_diagonal(), [1.0, 4.0]);
        m7.set_diagonal([5.0, 6.0]);
        assert_eq!(m7.get_diagonal(), [5.0, 6.0]);
        println!("m7: {}", m7);
    }

    #[test]
    fn test_display() {
        let m8 = Matrix2::from_values(1.0, 2.0, 3.0, 4.0);
        assert_eq!(format!("{}", m8), "Matrix2([[1, 2], [3, 4]])");
    }

    #[test]
    fn test_is_equal() {
        let m9 = Matrix2::from_values(1.0, 2.0, 3.0, 4.0);
        let m10 = Matrix2::from_values(1.0, 2.0, 3.0, 4.0);
        let m11 = Matrix2::from_values(1.1, 2.2, 3.3, 4.4);

        assert!(m9.is_equal(&m10, 1e-6));
        assert!(!m9.is_equal(&m11, 1e-6));
    }
}
