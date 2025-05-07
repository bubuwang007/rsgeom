use crate::fconst::FloatWithConst;
use crate::xyz::XYZ;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3d<T = f64> {
    xyz: XYZ<T>,
}

impl<T> std::fmt::Display for Point3d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point3d({}, {}, {})", self.xyz.x, self.xyz.y, self.xyz.z)
    }
}

impl<T> Point3d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Point3d { xyz: XYZ::new() }
    }

    pub fn from_xyz(xyz: XYZ<T>) -> Self {
        Point3d { xyz }
    }

    pub fn from_coordinates(x: T, y: T, z: T) -> Self {
        Point3d {
            xyz: XYZ::from_coordinates(x, y, z),
        }
    }

    pub fn x(&self) -> T {
        self.xyz.x
    }

    pub fn set_x(&mut self, x: T) {
        self.xyz.x = x;
    }

    pub fn y(&self) -> T {
        self.xyz.y
    }

    pub fn set_y(&mut self, y: T) {
        self.xyz.y = y;
    }

    pub fn z(&self) -> T {
        self.xyz.z
    }

    pub fn set_z(&mut self, z: T) {
        self.xyz.z = z;
    }

    pub fn xyz(&self) -> &XYZ<T> {
        &self.xyz
    }

    pub fn set_xyz(&mut self, xyz: XYZ<T>) {
        self.xyz = xyz;
    }

    pub fn coord(&self) -> (T, T, T) {
        (self.xyz.x, self.xyz.y, self.xyz.z)
    }

    pub fn set_coord(&mut self, x: T, y: T, z: T) {
        self.xyz.x = x;
        self.xyz.y = y;
        self.xyz.z = z;
    }

    pub fn distance(&self, other: &Self) -> T {
        ((self.xyz.x - other.xyz.x).powi(2)
            + (self.xyz.y - other.xyz.y).powi(2)
            + (self.xyz.z - other.xyz.z).powi(2))
        .sqrt()
    }

    pub fn square_distance(&self, other: &Self) -> T {
        (self.xyz.x - other.xyz.x).powi(2)
            + (self.xyz.y - other.xyz.y).powi(2)
            + (self.xyz.z - other.xyz.z).powi(2)
    }

    pub fn is_equal(&self, other: &Self, tolerance: T) -> bool {
        self.distance(other) <= tolerance
    }

    pub fn bary_center(&mut self, alpha: T, p: &Self, beta: T) {
        let xyz = *self.xyz();
        self.xyz.set_linear_form_2(alpha, &xyz, beta, p.xyz());
        self.xyz /= alpha + beta;
    }

    pub fn mirror_by_point3d(&mut self, p: &Self) {
        self.xyz.reverse();
        let mut xyz1: XYZ<T> = p.xyz;
        xyz1 *= T::from(2.0).unwrap();
        self.xyz += &xyz1;
    }

    pub fn mirror_by_point3d_new(&self, p: &Self) -> Self {
        let mut result = *self;
        result.mirror_by_point3d(p);
        result
    }

    // pub fn mirror_by_ax1 (){}
    // pub fn mirror_by_ax2 (){}
    // pub fn rotate_by_ax1 (){}

    pub fn scale_by_point3d(&mut self, p: &Self, scale: T) {
        let mut xyz1: XYZ<T> = p.xyz;
        xyz1 *= T::from(1.0).unwrap() - scale;
        self.xyz *= T::from(scale).unwrap();
        self.xyz += &xyz1;
    }

    pub fn scale_by_point3d_new(&self, p: &Self, scale: T) -> Self {
        let mut result = *self;
        result.scale_by_point3d(p, scale);
        result
    }

    pub fn translate_by_2point3d(&mut self, p1: &Self, p2: &Self) {
        self.xyz += &p2.xyz;
        self.xyz -= &p1.xyz;
    }

    pub fn translate_by_2point3d_new(&self, p1: &Self, p2: &Self) -> Self {
        let mut result = *self;
        result.translate_by_2point3d(p1, p2);
        result
    }

    // pub fn translate_by_vec2d(){}
    // pub fn translate_by_vec2d_new(){}
    // pub fn transform() {}
    // pub fn transform_new() {}

}

use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Point3d<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.xyz.x,
            1 => &self.xyz.y,
            2 => &self.xyz.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Point3d<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.xyz.x,
            1 => &mut self.xyz.y,
            2 => &mut self.xyz.z,
            _ => panic!("Index out of bounds"),
        }
    }
}
