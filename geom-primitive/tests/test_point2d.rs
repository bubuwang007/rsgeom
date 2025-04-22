use geom_primitive::point2d;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let p1 = point2d::Point2d { x: 1.0, y: 2.0 };
        assert_eq!(p1.x, 1.0);
        assert_eq!(p1.y, 2.0);
    }
}
