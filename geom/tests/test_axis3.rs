use geom::Axis3d;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis3_new() {
        let axis = Axis3d::<f64>::new();
        assert_eq!(axis.location.get_x(), 0.0);
        assert_eq!(axis.location.get_y(), 0.0);
        assert_eq!(axis.location.get_z(), 0.0);
        assert_eq!(axis.direction.get_x(), 0.0);
        assert_eq!(axis.direction.get_y(), 0.0);
        assert_eq!(axis.direction.get_z(), 0.0);
        println!("axis: {}", axis);
    }

    #[test]
    fn test_axis3_from_location_direction() {
        let axis = Axis3d::from_location_direction((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));
        assert_eq!(axis.location.get_x(), 1.0);
        assert_eq!(axis.location.get_y(), 2.0);
        assert_eq!(axis.location.get_z(), 3.0);
        assert_eq!(axis.direction.get_x(), 4.0);
        assert_eq!(axis.direction.get_y(), 5.0);
        assert_eq!(axis.direction.get_z(), 6.0);
        println!("axis: {}", axis);
    }

    #[test]
    fn test_axis3_set_location() {
        let mut axis = Axis3d::new();
        axis.set_location((7.0, 8.0, 9.0));
        assert_eq!(axis.location.get_x(), 7.0);
        assert_eq!(axis.location.get_y(), 8.0);
        assert_eq!(axis.location.get_z(), 9.0);
        assert_eq!(axis.direction.get_x(), 0.0);
        assert_eq!(axis.direction.get_y(), 0.0);
        assert_eq!(axis.direction.get_z(), 0.0);
        println!("axis: {}", axis);
    }

    #[test]
    fn test_axis3_set_direction() {
        let mut axis = Axis3d::new();
        axis.set_direction((10.0, 11.0, 12.0));
        assert_eq!(axis.location.get_x(), 0.0);
        assert_eq!(axis.location.get_y(), 0.0);
        assert_eq!(axis.location.get_z(), 0.0);
        assert_eq!(axis.direction.get_x(), 10.0);
        assert_eq!(axis.direction.get_y(), 11.0);
        assert_eq!(axis.direction.get_z(), 12.0);
        println!("axis: {}", axis);
    }

}