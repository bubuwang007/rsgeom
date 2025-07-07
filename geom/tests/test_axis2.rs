use geom::Axis2;

#[cfg(test)]
mod tests {
    use geom::{Direction2d, Point2d};

    use super::*;

    #[test]
    fn test_axis2_new() {
        let axis = Axis2::<f64>::new();
        assert_eq!(axis.location.get_x(), 0.0);
        assert_eq!(axis.location.get_y(), 0.0);
        assert_eq!(axis.direction.get_x(), 0.0);
        assert_eq!(axis.direction.get_y(), 0.0);
        println!("axis: {}", axis);
    }

    #[test]
    fn test_axis2_from_location_direction() {
        let axis = Axis2::from_location_direction(
            Point2d::from_coords(1.0, 2.0), 
            Direction2d::from_coords(3.0, 4.0)
        );

        // let axis = Axis2::from_location_direction((1.0, 2.0), (3.0, 4.0));
        assert_eq!(axis.location.get_x(), 1.0);
        assert_eq!(axis.location.get_y(), 2.0);
        assert_eq!(axis.direction.get_x(), 3.0);
        assert_eq!(axis.direction.get_y(), 4.0);
        println!("axis: {}", axis);
    }
}