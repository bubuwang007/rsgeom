use geom::Direction3d;
use geom::XYZ;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let d1 = Direction3d::<f64>::new();
        assert_eq!(d1.xyz.x, 0.0);
        assert_eq!(d1.xyz.y, 0.0);
        assert_eq!(d1.xyz.z, 0.0);
        println!("d1: {}", d1);
    }

    #[test]
    fn test_from_xyz() {
        let xyz = XYZ::from_coords(1.0, 2.0, 3.0);
        let d2 = Direction3d::from_xyz(xyz);
        assert_eq!(d2.xyz.x, 1.0);
        assert_eq!(d2.xyz.y, 2.0);
        assert_eq!(d2.xyz.z, 3.0);
    }

    #[test]
    fn test_from_coords() {
        let d3 = Direction3d::from_coords(4.0, 5.0, 6.0);
        assert_eq!(d3.xyz.x, 4.0);
        assert_eq!(d3.xyz.y, 5.0);
        assert_eq!(d3.xyz.z, 6.0);
    }

    #[test]
    fn test_coords() {
        let mut d4 = Direction3d::from_coords(7.0, 8.0, 9.0);
        assert_eq!(d4.xyz.get_coords(), (7.0, 8.0, 9.0));

        d4.set_coords(10.0, 20.0, 30.0);
        assert_eq!(d4.xyz.get_coords(), (10.0, 20.0, 30.0));
    }

    #[test]
    fn test_x_y_z() {
        let mut d5 = Direction3d::from_coords(11.0, 12.0, 13.0);
        assert_eq!(d5.xyz.get_x(), 11.0);
        assert_eq!(d5.xyz.get_y(), 12.0);
        assert_eq!(d5.xyz.get_z(), 13.0);

        d5.set_x(14.0);
        d5.set_y(15.0);
        d5.set_z(16.0);
        assert_eq!(d5.xyz.get_x(), 14.0);
        assert_eq!(d5.xyz.get_y(), 15.0);
        assert_eq!(d5.xyz.get_z(), 16.0);
    }

    #[test]
    fn test_xyz() {
        let mut d = Direction3d::from_xyz(XYZ::from_coords(17.0, 18.0, 19.0));
        assert_eq!(d.xyz.get_coords(), (17.0, 18.0, 19.0));
        d.set_xyz(XYZ::from_coords(20.0, 21.0, 22.0));
        assert_eq!(d.xyz.get_coords(), (20.0, 21.0, 22.0));
    }

    #[test]
    fn test_is_equal() {
        let d1 = Direction3d::from_coords(1.0, 2.0, 3.0);
        let d2 = Direction3d::from_coords(1.0, 2.0, 3.0);
        let d3 = Direction3d::from_coords(1.1, 2.1, 3.1);

        assert!(d1.is_equal(&d2, 1e-6));
        assert!(!d1.is_equal(&d3, 1e-6));
    }

    #[test]
    fn test_display() {
        let d = Direction3d::from_coords(4.0, 5.0, 6.0);
        assert_eq!(format!("{}", d), "Direction3d(4, 5, 6)");
    }
}
