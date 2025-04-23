use geom_primitive::point2d::Point2d;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {

        let p2 = Point2d::new();
        assert_eq!(p2.x(), 0.0);
        assert_eq!(p2.y(), 0.0);



        let p3 = Point2d::from_coordinates(3.0, 4.0);
        assert_eq!(p3.x(), 3.0);
        assert_eq!(p3.y(), 4.0);
    }

    #[test]
    fn test_index() {
        let p = Point2d::from_coordinates(1.0, 2.0);
        assert_eq!(p[0], 1.0);
        assert_eq!(p[1], 2.0);

        let mut p_mut = p;
        p_mut[0] = 3.0;
        p_mut[1] = 4.0;
        assert_eq!(p_mut[0], 3.0);
        assert_eq!(p_mut[1], 4.0);
    }

    #[test]
    fn test_coord() {
        let mut p = Point2d::new();
        p.set_coordinates(5.0, 6.0);
        assert_eq!(p.x(), 5.0);
        assert_eq!(p.y(), 6.0);

        let (x, y) = p.coord();
        assert_eq!(x, 5.0);
        assert_eq!(y, 6.0);
    }

    #[test]
    fn test_change_coord() {
        let mut p = Point2d::from_coordinates(1.0, 2.0);
        let x_coord = p.change_coord();
        x_coord.x = 3.0;
        assert_eq!(p.x(), 3.0);
        assert_eq!(p.y(), 2.0);
    }

    #[test]
    fn test_distance() {
        let p1 = Point2d::from_coordinates(1.0, 2.0);
        let p2 = Point2d::from_coordinates(4.0, 6.0);
        let distance = p1.distance(&p2);
        assert_eq!(distance, 5.0);
        let distance_squared = p1.distance_squared(&p2);
        assert_eq!(distance_squared, 25.0);
    }

    #[test]
    fn test_is_equal() {
        let p1 = Point2d::from_coordinates(1.0, 2.0);
        let p2 = Point2d::from_coordinates(1.0, 2.0);
        let p3 = Point2d::from_coordinates(1.0, 3.0);
        assert!(p1.is_equal(&p2, 1e-6));
        assert!(!p1.is_equal(&p3, 1e-6));
    }

    #[test]
    fn test_mirror_by_point2d() {
        let mut p1 = Point2d::from_coordinates(1.0, 2.0);
        let p2 = Point2d::from_coordinates(3.0, 4.0);
        p1.mirror_by_point2d(&p2);
        assert_eq!(p1.x(), 5.0);
        assert_eq!(p1.y(), 6.0);
    }

    #[test]
    fn test_scale_by_point2d(){
        let mut p1 = Point2d::from_coordinates(1.0, 2.0);
        let p2 = Point2d::from_coordinates(3.0, 4.0);
        p1.scale_by_point2d(&p2, 0.5);
        assert_eq!(p1.x(), 2.0);
        assert_eq!(p1.y(), 3.0);
    }

}
