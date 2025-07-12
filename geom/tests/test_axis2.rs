use geom::Axis2d;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis2_new() {
        let axis = Axis2d::<f64>::new();
        assert_eq!(axis.location.get_x(), 0.0);
        assert_eq!(axis.location.get_y(), 0.0);
        assert_eq!(axis.direction.get_x(), 0.0);
        assert_eq!(axis.direction.get_y(), 0.0);
        println!("axis: {}", axis);
    }

    #[test]
    fn test_axis2_from_location_direction() {
        let axis = Axis2d::from_location_direction((1.0, 2.0), (3.0, 4.0));
        assert_eq!(axis.location.get_x(), 1.0);
        assert_eq!(axis.location.get_y(), 2.0);
        assert_eq!(axis.direction.get_x(), 3.0);
        assert_eq!(axis.direction.get_y(), 4.0);
        println!("axis: {}", axis);
    }

    #[test]
    fn test_axis2_set_location() {
        let mut axis = Axis2d::new();
        axis.set_location((5.0, 6.0));
        assert_eq!(axis.location.get_x(), 5.0);
        assert_eq!(axis.location.get_y(), 6.0);
        assert_eq!(axis.direction.get_x(), 0.0);
        assert_eq!(axis.direction.get_y(), 0.0);
        println!("axis: {}", axis);
    }

    #[test]
    fn test_axis2_set_direction() {
        let mut axis = Axis2d::new();
        axis.set_direction((7.0, 8.0));
        assert_eq!(axis.location.get_x(), 0.0);
        assert_eq!(axis.location.get_y(), 0.0);
        assert_eq!(axis.direction.get_x(), 7.0);
        assert_eq!(axis.direction.get_y(), 8.0);
        println!("axis: {}", axis);
    }
}
