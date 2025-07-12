use geom::{Axis3d, CoordinateSystem3d};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_system2_new() {
        let cs = CoordinateSystem3d::<f64>::new();
        assert_eq!(cs.axis.location.get_coords(), (0.0, 0.0, 0.0));
        assert_eq!(cs.axis.direction.get_coords(), (0.0, 0.0, 0.0));

        assert_eq!(cs.vydir.get_coords(), (0.0, 0.0, 0.0));
        assert_eq!(cs.vxdir.get_coords(), (0.0, 0.0, 0.0));

        println!("CoordinateSystem2: {}", cs);
    }

    #[test]
    fn test_coordinate_system2_from_axis_vydir_vxdir() {
        let cs = CoordinateSystem3d::from_axis_vydir_vxdir(
            Axis3d::from_location_direction((0.0, 0.0, 0.0), (0.0, 0.0, 1.0)),
            (0.0, 1.0, 0.0), // VY direction
            (1.0, 0.0, 0.0), // VX direction
        );

        assert_eq!(cs.axis.location.get_coords(), (0.0, 0.0, 0.0));
        assert_eq!(cs.axis.direction.get_coords(), (0.0, 0.0, 1.0));

        assert_eq!(cs.vydir.get_coords(), (0.0, 1.0, 0.0));
        assert_eq!(cs.vxdir.get_coords(), (1.0, 0.0, 0.0));
        println!("{}", cs);
    }

    #[test]
    fn test_coordinate_system2_set_axis() {
        let mut cs = CoordinateSystem3d::new();
        cs.set_axis(Axis3d::from_location_direction(
            (1.0, 2.0, 3.0),
            (4.0, 5.0, 6.0),
        ));
        assert_eq!(cs.axis.location.get_coords(), (1.0, 2.0, 3.0));
        assert_eq!(cs.axis.direction.get_coords(), (4.0, 5.0, 6.0));
        println!("CoordinateSystem2: {}", cs);
    }
}
