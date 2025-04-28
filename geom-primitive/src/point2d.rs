use crate::xy::XY;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d {
    xy: XY,
}

impl Point2d {
    pub fn new() -> Self {
        Point2d { xy: XY::new() }
    }

    pub fn from_xy(xy: XY) -> Self {
        Point2d { xy }
    }

    pub fn from_coordinates(x: f64, y: f64) -> Self {
        Point2d {
            xy: XY::from_coordinates(x, y),
        }
    }

    pub fn to_string(&self) -> String {
        format!("Point2d({}, {})", self.xy.x, self.xy.y)
    }

    pub fn x(&self) -> f64 {
        self.xy.x
    }

    pub fn set_x(&mut self, x: f64) {
        self.xy.x = x;
    }

    pub fn y(&self) -> f64 {
        self.xy.y
    }

    pub fn set_y(&mut self, y: f64) {
        self.xy.y = y;
    }

    pub fn xy(&self) -> &XY {
        &self.xy
    }

    pub fn set_xy(&mut self, xy: XY) {
        self.xy = xy;
    }

    pub fn coord(&self) -> (f64, f64) {
        (self.xy.x, self.xy.y)
    }

    pub fn set_coord(&mut self, x: f64, y: f64) {
        self.xy.x = x;
        self.xy.y = y;
    }

    pub fn distance(&self, other: &Self) -> f64 {
        ((self.xy.x - other.xy.x).powi(2) + (self.xy.y - other.xy.y).powi(2)).sqrt()
    }

    pub fn square_distance(&self, other: &Self) -> f64 {
        (self.xy.x - other.xy.x).powi(2) + (self.xy.y - other.xy.y).powi(2)
    }

    pub fn is_equal(&self, other: &Self, tolerance: f64) -> bool {
        self.distance(other) <= tolerance
    }

    pub fn mirror_by_point2d(&mut self, other: &Self) {
        self.xy.reverse();
        let mut xy1 = other.xy;
        xy1 *= 2.0;
        self.xy += &xy1;
    }

    pub fn mirror_by_point2d_new(&self, other: &Self) -> Self {
        let mut result = *self;
        result.mirror_by_point2d(other);
        result
    }

    // Todo: mirror_by_ax2d

    // pub fn rotate_by_point2d(&mut self, other: &Self, angle: f64) {
    // }

    pub fn scale_by_point2d(&mut self, other: &Self, scale: f64) {
        let mut xy1 = other.xy;
        xy1 *= 1.0 - scale;
        self.xy *= scale;
        self.xy += &xy1;
    }

    // pub fn translate_by_vec2d(){}
    // pub fn translate_by_vec2d_new(){}
    // pub fn transform


}

use std::ops::{Index, IndexMut};

impl Index<usize> for Point2d {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.xy.x,
            1 => &self.xy.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Point2d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.xy.x,
            1 => &mut self.xy.y,
            _ => panic!("Index out of bounds"),
        }
    }
}
