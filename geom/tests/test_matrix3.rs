use geom::Matrix3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m1 = Matrix3::<f64>::new();
        assert_eq!(
            m1.get_values(),
            [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]
        );
        println!("m1: {}", m1);
    }

    #[test]
    fn test_from_array() {
        let m2 = Matrix3::from_array([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert_eq!(
            m2.get_values(),
            [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]
        );
        println!("m2: {}", m2);
    }

    #[test]
    fn test_from_values() {
        let m3 = Matrix3::from_values(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(
            m3.get_values(),
            [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]
        );
        println!("m3: {}", m3);
    }

    #[test]
    fn test_get_set_values() {
        let mut m4 = Matrix3::new();
        m4.set_values(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(
            m4.get_values(),
            [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]
        );
        println!("m4: {}", m4);
    }

    #[test]
    fn test_get_set_col() {
        let mut m5 = Matrix3::from_values(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m5.get_col(0), [1.0, 4.0, 7.0]);
        m5.set_col(0, [10.0, 11.0, 12.0]);
        assert_eq!(m5.get_col(0), [10.0, 11.0, 12.0]);
        println!("m5: {}", m5);
    }

    #[test]
    fn test_get_set_row() {
        let mut m6 = Matrix3::from_values(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m6.get_row(1), [4.0, 5.0, 6.0]);
        m6.set_row(1, [10.0, 11.0, 12.0]);
        assert_eq!(m6.get_row(1), [10.0, 11.0, 12.0]);
        println!("m6: {}", m6);
    }

    #[test]
    fn test_get_set_diagonal() {
        let mut m7 = Matrix3::from_values(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m7.get_diagonal(), [1.0, 5.0, 9.0]);
        m7.set_diagonal([10.0, 11.0, 12.0]);
        assert_eq!(m7.get_diagonal(), [10.0, 11.0, 12.0]);
        println!("m7: {}", m7);
    }

    #[test]
    fn test_display() {
        let m8 = Matrix3::from_values(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(
            format!("{}", m8),
            "Matrix3([[1, 2, 3], [4, 5, 6], [7, 8, 9]])"
        );
        println!("m8: {}", m8);
    }

    #[test]
    fn test_is_equal() {
        let m1 = Matrix3::from_values(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let m2 = Matrix3::from_values(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let m3 = Matrix3::from_values(1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 8.1, 9.1);

        assert!(m1.is_equal(&m2, 1e-5));
        assert!(!m1.is_equal(&m3, 1e-5));
    }
}
