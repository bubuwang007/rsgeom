use geom_primitive::Point3d;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let p3: Point3d<f64> = Point3d::new();
        assert_eq!(p3.x(), 0.0);
        assert_eq!(p3.y(), 0.0);
        assert_eq!(p3.z(), 0.0);

        let p4 = Point3d::from_coordinates(3.0, 4.0, 5.0);
        assert_eq!(p4.x(), 3.0);
        assert_eq!(p4.y(), 4.0);
        assert_eq!(p4.z(), 5.0);
    }

    #[test]
    fn test_to_string() {
        let p = Point3d::from_coordinates(1.0, 2.0, 3.0);
        assert_eq!(p.to_string(), "Point3d(1, 2, 3)");
    }

    #[test]
    fn test_index() {
        let p = Point3d::from_coordinates(1.0, 2.0, 3.0);
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
    fn test_coord() {
        let mut p = Point3d::new();
        p.set_coord(5.0, 6.0, 7.0);
        assert_eq!(p.x(), 5.0);
        assert_eq!(p.y(), 6.0);
        assert_eq!(p.z(), 7.0);

        let (x, y, z) = p.coord();
        assert_eq!(x, 5.0);
        assert_eq!(y, 6.0);
        assert_eq!(z, 7.0);
    }

    #[test]
    fn test_distance() {
        let p1 = Point3d::from_coordinates(1.0, 2.0, 3.0);
        let p2 = Point3d::from_coordinates(4.0, 5.0, 6.0);
        let distance = p1.distance(&p2);
        assert_eq!(distance, 5.196152422706632);
        let distance_squared = p1.square_distance(&p2);
        assert_eq!(distance_squared, 27.0);
    }

    #[test]
    fn test_is_equal() {
        let p1 = Point3d::from_coordinates(1.0, 2.0, 3.0);
        let p2 = Point3d::from_coordinates(1.0, 2.0, 3.0);
        let p3 = Point3d::from_coordinates(1.0, 2.0, 4.0);
        assert!(p1.is_equal(&p2, 1e-6));
        assert!(!p1.is_equal(&p3, 1e-6));
    }

    #[test]
    fn test_bary_center() {
        let mut p1 = Point3d::from_coordinates(1.0, 2.0, 3.0);
        let p2 = Point3d::from_coordinates(4.0, 5.0, 6.0);
        p1.bary_center(2.0, &p2, 3.0);
        assert_eq!(p1.x(), 2.8);
        assert_eq!(p1.y(), 3.8);
        assert_eq!(p1.z(), 4.8);
    }

    #[test]
    fn test_mirror_by_point3d() {
        let mut p1 = Point3d::from_coordinates(1.0, 2.0, 3.0);
        let p2 = Point3d::from_coordinates(4.0, 5.0, 6.0);

        let p3 = p1.mirror_by_point3d_new(&p2);
        assert_eq!(p3.x(), 7.0);
        assert_eq!(p3.y(), 8.0);
        assert_eq!(p3.z(), 9.0);

        p1.mirror_by_point3d(&p2);
        assert_eq!(p1.x(), 7.0);
        assert_eq!(p1.y(), 8.0);
        assert_eq!(p1.z(), 9.0);
    }

    #[test]
    fn test_scale() {
        let mut p1 = Point3d::from_coordinates(1.0, 2.0, 3.0);
        let p2 = Point3d::from_coordinates(4.0, 5.0, 6.0);

        let p3 = p1.scale_by_point3d_new(&p2, 0.5);
        assert_eq!(p3.x(), 2.5);
        assert_eq!(p3.y(), 3.5);
        assert_eq!(p3.z(), 4.5);

        p1.scale_by_point3d(&p2, 0.5);
        assert_eq!(p1.x(), 2.5);
        assert_eq!(p1.y(), 3.5);
        assert_eq!(p1.z(), 4.5);
    }

    #[test]
    fn test_translate() {
        let mut p1 = Point3d::from_coordinates(1.0, 2.0, 3.0);
        let p2 = Point3d::from_coordinates(4.0, 5.0, 6.0);
        let p3 = Point3d::from_coordinates(7.0, 8.0, 9.0);

        p1.translate_by_2point3d(&p2, &p3);
        assert_eq!(p1.x(), 4.0);
        assert_eq!(p1.y(), 5.0);
        assert_eq!(p1.z(), 6.0);

        let p4 = p1.translate_by_2point3d_new(&p2, &p3);
        assert_eq!(p4.x(), 7.0);
        assert_eq!(p4.y(), 8.0);
        assert_eq!(p4.z(), 9.0);
    }
}
