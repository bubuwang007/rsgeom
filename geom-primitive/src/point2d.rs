use crate::fconst::FloatWithConst;
use crate::xy::XY;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d<T = f64> {
    xy: XY<T>,
}

impl<T> std::fmt::Display for Point2d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point2d({}, {})", self.xy.x, self.xy.y)
    }
}

impl<T> Point2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Point2d { xy: XY::new() }
    }

    pub fn from_xy(xy: XY<T>) -> Self {
        Point2d { xy }
    }

    pub fn from_coordinates(x: T, y: T) -> Self {
        Point2d {
            xy: XY::from_coordinates(x, y),
        }
    }

    pub fn x(&self) -> T {
        self.xy.x
    }

    pub fn set_x(&mut self, x: T) {
        self.xy.x = x;
    }

    pub fn y(&self) -> T {
        self.xy.y
    }

    pub fn set_y(&mut self, y: T) {
        self.xy.y = y;
    }

    pub fn xy(&self) -> &XY<T> {
        &self.xy
    }

    pub fn set_xy(&mut self, xy: XY<T>) {
        self.xy = xy;
    }

    pub fn coord(&self) -> (T, T) {
        (self.xy.x, self.xy.y)
    }

    pub fn set_coord(&mut self, x: T, y: T) {
        self.xy.x = x;
        self.xy.y = y;
    }

    pub fn distance(&self, other: &Self) -> T {
        ((self.xy.x - other.xy.x).powi(2) + (self.xy.y - other.xy.y).powi(2)).sqrt()
    }

    pub fn square_distance(&self, other: &Self) -> T {
        (self.xy.x - other.xy.x).powi(2) + (self.xy.y - other.xy.y).powi(2)
    }

    pub fn is_equal(&self, other: &Self, tolerance: T) -> bool {
        self.distance(other) <= tolerance
    }

    pub fn mirror_by_point2d(&mut self, other: &Self) {
        self.xy.reverse();
        let mut xy1: XY<T> = other.xy;
        xy1 *= T::from(2.0).unwrap();
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

    pub fn scale_by_point2d(&mut self, other: &Self, scale: T) {
        let mut xy1 = other.xy;
        xy1 *= T::from(1.0).unwrap() - scale;
        self.xy *= T::from(scale).unwrap();
        self.xy += &xy1;
    }

    pub fn scale_by_point2d_new(&self, other: &Self, scale: T) -> Self {
        let mut result = *self;
        result.scale_by_point2d(other, scale);
        result
    }

    pub fn translate_by_2point2d(&mut self, p1: &Self, p2: &Self) {
        self.xy += &p2.xy;
        self.xy -= &p1.xy;
    }

    pub fn translate_by_2point2d_new(&self, p1: &Self, p2: &Self) -> Self {
        let mut result = *self;
        result.translate_by_2point2d(p1, p2);
        result
    }

    // pub fn translate_by_vec2d(){}
    // pub fn translate_by_vec2d_new(){}
    // pub fn transform() {}
    // pub fn transform_new() {}
}

use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Point2d<T>
where
    T: Copy + FloatWithConst,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.xy.x,
            1 => &self.xy.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Point2d<T>
where
    T: Copy + FloatWithConst,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.xy.x,
            1 => &mut self.xy.y,
            _ => panic!("Index out of bounds"),
        }
    }
}
