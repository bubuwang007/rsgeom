use geom::Direction3d;
use geom::XYZ;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let d1 = Direction3d::<f64>::new();
        assert_eq!(d1.xyz.x, 1.0);
        assert_eq!(d1.xyz.y, 0.0);
        assert_eq!(d1.xyz.z, 0.0);
        println!("d1: {}", d1);
    }

    #[test]
    fn test_from_xyz() {
        let xyz = XYZ::from_coords(1.0, 2.0, 3.0);
        let d2 = Direction3d::from_xyz(xyz);
        assert_eq!(d2.xyz.x, 0.2672612419124244);
        assert_eq!(d2.xyz.y, 0.5345224838248488);
        assert_eq!(d2.xyz.z, 0.8017837257372732);
    }

    #[test]
    fn test_from_coords() {
        let d3 = Direction3d::from_coords(2.0, 3.0, 6.0);
        assert_eq!(d3.xyz.x, 2.0 / 7.0);
        assert_eq!(d3.xyz.y, 3.0 / 7.0);
        assert_eq!(d3.xyz.z, 6.0 / 7.0);
    }

    #[test]
    fn test_coords() {
        let mut d4 = Direction3d::from_coords(1.0, 2.0, 2.0);
        assert_eq!(d4.xyz.get_coords(), (1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0));

        d4.set_coords(2.0, 3.0, 6.0);
        assert_eq!(d4.xyz.get_coords(), (2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0));
    }

    #[test]
    fn test_x_y_z() {
        let mut d5 = Direction3d::from_coords(1.0, 2.0, 2.0);
        assert_eq!(d5.xyz.get_x(), 1.0 / 3.0);
        assert_eq!(d5.xyz.get_y(), 2.0 / 3.0);
        assert_eq!(d5.xyz.get_z(), 2.0 / 3.0);

        d5.set_coords(2.0, 3.0, 6.0);
        assert_eq!(d5.xyz.get_x(), 2.0 / 7.0);
        assert_eq!(d5.xyz.get_y(), 3.0 / 7.0);
        assert_eq!(d5.xyz.get_z(), 6.0 / 7.0);
    }

    #[test]
    fn test_xyz() {
        let mut d = Direction3d::from_xyz(XYZ::from_coords(1.0, 2.0, 2.0));
        assert_eq!(d.xyz.get_coords(), (1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0));
        d.set_xyz(XYZ::from_coords(2.0, 3.0, 6.0));
        assert_eq!(d.xyz.get_coords(), (2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0));
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
        assert_eq!(
            format!("{}", d),
            "Direction3d(0.4558423058385518, 0.5698028822981898, 0.6837634587578276)"
        );
    }

    #[test]
    fn test_angle() {
        let d1 = Direction3d::from_coords(1.0, 0.0, 0.0);
        let d2 = Direction3d::from_coords(0.0, 1.0, 0.0);
        let d3 = Direction3d::from_coords(1.0, 1.0, 1.0);

        let angle12 = d1.angle(&d2);
        let angle13 = d1.angle(&d3);

        assert!((angle12 - std::f64::consts::FRAC_PI_2).abs() < 1e-6);
        assert!((angle13 - 0.955).abs() < 1e-3);
    }
}
