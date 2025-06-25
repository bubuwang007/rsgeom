use geom::Point3d;
use geom::XYZ;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let point: Point3d<f64> = Point3d::new();
        assert_eq!(point.get_x(), 0.0);
        assert_eq!(point.get_y(), 0.0);
        assert_eq!(point.get_z(), 0.0);
    }

    #[test]
    fn test_from_xyz() {
        let xyz = XYZ::from_coords(1.0, 2.0, 3.0);
        let point = Point3d::from_xyz(xyz);
        assert_eq!(point.get_x(), 1.0);
        assert_eq!(point.get_y(), 2.0);
        assert_eq!(point.get_z(), 3.0);
    }

    #[test]
    fn test_from_coords() {
        let point = Point3d::from_coords(4.0, 5.0, 6.0);
        assert_eq!(point.get_x(), 4.0);
        assert_eq!(point.get_y(), 5.0);
        assert_eq!(point.get_z(), 6.0);
    }

    #[test]
    fn test_coords() {
        let mut point = Point3d::new();
        point.set_coords(7.0, 8.0, 9.0);
        assert_eq!(point.get_coords(), (7.0, 8.0, 9.0));
    }

    #[test]
    fn test_xyz() {
        let mut point = Point3d::new();
        let xyz = XYZ::from_coords(10.0, 11.0, 12.0);
        point.set_xyz(xyz);
        let xyz = XYZ::from_coords(10.0, 11.0, 12.0);
        assert_eq!(point.get_xyz().is_equal(&xyz, 1e-6), true);
    }
}