use geom::Vector3d;
use geom::XYZ;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v1 = Vector3d::<f64>::new();
        assert_eq!(v1.xyz.x, 0.0);
        assert_eq!(v1.xyz.y, 0.0);
        assert_eq!(v1.xyz.z, 0.0);
        println!("v1: {}", v1);
    }

    #[test]
    fn test_from_xyz() {
        let xyz = XYZ::from_coords(1.0, 2.0, 3.0);
        let v2 = Vector3d::from_xyz(xyz);
        assert_eq!(v2.xyz.x, 1.0);
        assert_eq!(v2.xyz.y, 2.0);
        assert_eq!(v2.xyz.z, 3.0);
    }

    #[test]
    fn test_from_coords() {
        let v3 = Vector3d::from_coords(4.0, 5.0, 6.0);
        assert_eq!(v3.xyz.x, 4.0);
        assert_eq!(v3.xyz.y, 5.0);
        assert_eq!(v3.xyz.z, 6.0);
    }

    #[test]
    fn test_coords() {
        let mut v4 = Vector3d::from_coords(7.0, 8.0, 9.0);
        assert_eq!(v4.xyz.get_coords(), (7.0, 8.0, 9.0));

        v4.set_coords(10.0, 20.0, 30.0);
        assert_eq!(v4.xyz.get_coords(), (10.0, 20.0, 30.0));
    }

    #[test]
    fn test_x_y_z() {
        let mut v5 = Vector3d::from_coords(11.0, 12.0, 13.0);
        assert_eq!(v5.xyz.get_x(), 11.0);
        assert_eq!(v5.xyz.get_y(), 12.0);
        assert_eq!(v5.xyz.get_z(), 13.0);

        v5.set_x(14.0);
        v5.set_y(15.0);
        v5.set_z(16.0);
        assert_eq!(v5.xyz.get_x(), 14.0);
        assert_eq!(v5.xyz.get_y(), 15.0);
        assert_eq!(v5.xyz.get_z(), 16.0);
    }

    #[test]
    fn test_xyz() {
        let mut v = Vector3d::from_xyz(XYZ::from_coords(17.0, 18.0, 19.0));
        assert_eq!(v.xyz.get_coords(), (17.0, 18.0, 19.0));
        v.set_xyz(XYZ::from_coords(20.0, 21.0, 22.0));
        assert_eq!(v.xyz.get_coords(), (20.0, 21.0, 22.0));
    }

    #[test]
    fn test_is_equal() {
        let v1 = Vector3d::from_coords(1.0, 2.0, 3.0);
        let v2 = Vector3d::from_coords(1.0, 2.0, 3.0);
        let v3 = Vector3d::from_coords(4.0, 5.0, 6.0);

        assert!(v1.is_equal(&v2, 1e-6));
        assert!(!v1.is_equal(&v3, 1e-6));
    }

    #[test]
    fn test_display() {
        let v = Vector3d::from_coords(23.0, 24.0, 25.0);
        assert_eq!(format!("{}", v), "Vector3d(23, 24, 25)");
    }
}
