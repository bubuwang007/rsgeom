use crate::point2d::Point2d;
use crate::xy::XY;
// use num_traits::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2d<T = f64> {
    xy: XY<T>,
}

impl Vector2d<f64> {
    pub const VX: Vector2d = Vector2d {
        xy: XY { x: 1.0, y: 0.0 },
    };

    pub const VY: Vector2d = Vector2d {
        xy: XY { x: 0.0, y: 1.0 },
    };
}

impl Vector2d<f32> {
    pub const VX: Vector2d<f32> = Vector2d {
        xy: XY { x: 1.0, y: 0.0 },
    };

    pub const VY: Vector2d<f32> = Vector2d {
        xy: XY { x: 0.0, y: 1.0 },
    };
}

impl<T> std::fmt::Display for Vector2d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2d({}, {})", self.xy.x, self.xy.y)
    }
}

impl<T> Vector2d<T>
where
    T: Copy + Default + crate::fconst::FloatWithConst,
{
    pub fn new() -> Self {
        Vector2d { xy: XY::new() }
    }

    // pub fn from_direction2d(x: f64, y: f64) -> Self {}

    pub fn from_coordinates(x: T, y: T) -> Self {
        Vector2d {
            xy: XY::from_coordinates(x, y),
        }
    }

    pub fn from_xy(xy: XY<T>) -> Self {
        Vector2d { xy }
    }

    pub fn from_2point2d(p1: &Point2d<T>, p2: &Point2d<T>) -> Self {
        Vector2d {
            xy: XY::from_coordinates(p2.x() - p1.x(), p2.y() - p1.y()),
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

    pub fn length(&self) -> T {
        self.xy.modulus()
    }

    pub fn square_length(&self) -> T {
        self.xy.square_modulus()
    }

    pub fn angle_to_vector(&self, other: &Self) -> Result<T, &'static str> {
        let mag = self.length();
        let other_mag = other.length();
        if mag <= T::min_positive_value() || other_mag <= T::min_positive_value() {
            return Err("Cannot calculate angle to zero vector");
        }
        let d = mag * other_mag;
        let cosinus = self.xy.dot(&other.xy()) / d;
        let sinus = self.xy.cross(&other.xy()) / d;

        let zero = T::from(0.0).unwrap();
        if cosinus > T::from(-0.70710678118655).unwrap()
            && cosinus < T::from(0.70710678118655).unwrap()
        {
            if sinus > zero {
                return Ok(cosinus.acos());
            } else {
                return Ok(-cosinus.acos());
            }
        } else {
            if cosinus > zero {
                return Ok(sinus.asin());
            } else {
                if sinus > zero {
                    return Ok(T::pi() - sinus.asin());
                } else {
                    return Ok(-T::pi() - sinus.asin());
                }
            }
        }
    }

    pub fn is_equal(
        &self,
        other: &Self,
        linear_tolerance: T,
        angular_tolerance: T,
    ) -> Result<bool, &'static str> {
        let mag = self.length();
        let other_mag = other.length();
        let val = (mag - other_mag).abs();

        let is_equal_length = val <= linear_tolerance;
        if (mag > linear_tolerance) && (other_mag > linear_tolerance) {
            let ang = self.angle_to_vector(other)?.abs();
            return Ok(is_equal_length && ang <= angular_tolerance);
        }
        Ok(is_equal_length)
    }

    pub fn is_orthogonal(&self, other: &Self, ang_tolerance: T) -> Result<bool, &'static str> {
        let ang = (T::frac_pi_2() - self.angle_to_vector(other)?.abs()).abs();
        if ang < ang_tolerance {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn is_opposite(&self, other: &Self, ang_tolerance: T) -> Result<bool, &'static str> {
        let ang = self.angle_to_vector(other)?.abs();
        if (T::pi() - ang).abs() < ang_tolerance {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn is_parallel(&self, other: &Self, ang_tolerance: T) -> Result<bool, &'static str> {
        let ang = self.angle_to_vector(other)?.abs();
        if (T::pi() - ang).abs() < ang_tolerance || ang < ang_tolerance {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn cross(&self, other: &Self) -> T {
        self.xy.cross(&other.xy())
    }

    pub fn cross_abs(&self, other: &Self) -> T {
        self.xy.cross_abs(&other.xy())
    }

    pub fn square_cross_abs(&self, other: &Self) -> T {
        self.xy.square_cross(&other.xy())
    }

    pub fn dot(&self, other: &Self) -> T {
        self.xy.dot(&other.xy())
    }

    pub fn normalize(&mut self) {
        self.xy.normalize();
    }

    pub fn normalize_new(&self) -> Self {
        let mut result = *self;
        result.normalize();
        result
    }

    pub fn reverse(&mut self) {
        self.xy.reverse();
    }

    pub fn reverse_new(&self) -> Self {
        let mut result = *self;
        result.reverse();
        result
    }

    // a1 * v1 + a2 * v2
    pub fn set_linear_form_2(&mut self, a1: T, v1: &Vector2d<T>, a2: T, v2: &Vector2d<T>) {
        self.xy.set_linear_form_2(a1, v1.xy(), a2, v2.xy());
    }

    // a1 * v1 + v2
    pub fn set_linear_form_2a(&mut self, a1: T, v1: &Vector2d<T>, v2: &Vector2d<T>) {
        self.xy.set_linear_form_2a(a1, v1.xy(), v2.xy());
    }

    // v1 + v2
    pub fn set_linear_form_2b(&mut self, v1: &Vector2d<T>, v2: &Vector2d<T>) {
        self.xy.set_linear_form_2b(v1.xy(), v2.xy());
    }

    // a1 * v1 + a2 * v2 + v3
    pub fn set_linear_form_3(
        &mut self,
        a1: T,
        v1: &Vector2d<T>,
        a2: T,
        v2: &Vector2d<T>,
        v3: &Vector2d<T>,
    ) {
        self.xy.set_linear_form_3(a1, v1.xy(), a2, v2.xy(), v3.xy());
    }

    // pub fn mirror_by_ax2d()

    // 存在疑问，结果永远是other
    pub fn mirror_by_vector2d(&mut self, other: &Self) {
        let d = other.xy().modulus();
        if d > T::min_positive_value() {
            let x = other.x();
            let y = other.y();
            let a = x / d;
            let b = y / d;
            let one = T::from(1.0).unwrap();
            let two = T::from(2.0).unwrap();
            let m1 = two * a * b;
            self.set_x(((two * a * a) - one) * x + m1 * y);
            self.set_y(m1 * x + ((two * b * b - one) * y));
        }
    }

    pub fn mirror_by_vector2d_new(&self, other: &Self) -> Self {
        let mut result = *self;
        result.mirror_by_vector2d(other);
        result
    }

    // // 需要trsf
    // pub fn rotate(&mut self, angle: f64) {

    // }

    pub fn scale(&mut self, scale: T) {
        self.xy *= scale;
    }

    pub fn scale_new(&self, scale: T) -> Self {
        let mut result = *self;
        result.scale(scale);
        result
    }

    // transform
}

use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Vector2d<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.xy.x,
            1 => &self.xy.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Vector2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.xy.x,
            1 => &mut self.xy.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

// use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use std::ops::Neg;

impl<T> Neg for &Vector2d<T> {
    type Output = Vector2d<T>;

    fn neg(self) -> Self::Output {
        Vector2d { xy: -&self.xy }
    }
}

use std::ops::{Add, AddAssign};

// Vector2d += &Vector2d
impl<T> AddAssign<&Vector2d<T>> for Vector2d<T> {
    fn add_assign(&mut self, other: &Vector2d<T>) {
        self.xy += &other.xy;
    }
}

// Vector2d += T
impl<T> AddAssign<T> for Vector2d<T> {
    fn add_assign(&mut self, other: T) {
        self.xy += other;
    }
}

impl<T> Add<&Vector2d<T>> for &Vector2d<T> {
    type Output = Vector2d<T>;

    fn add(self, other: &Vector2d<T>) -> Self::Output {
        Vector2d {
            xy: &self.xy + &other.xy,
        }
    }
}

impl<T> Add<T> for &Vector2d<T> {
    type Output = Vector2d<T>;

    fn add(self, other: T) -> Self::Output {
        Vector2d {
            xy: self.xy + other,
        }
    }
}

use std::ops::{Sub, SubAssign};

impl<T> SubAssign<&Vector2d<T>> for Vector2d<T> {
    fn sub_assign(&mut self, other: &Vector2d<T>) {
        self.xy -= &other.xy;
    }
}

impl SubAssign<f64> for Vector2d {
    fn sub_assign(&mut self, other: f64) {
        self.xy -= other;
    }
}

impl Sub<&Vector2d> for &Vector2d {
    type Output = Vector2d;

    fn sub(self, other: &Vector2d) -> Self::Output {
        Vector2d {
            xy: &self.xy - &other.xy,
        }
    }
}

impl Sub<f64> for &Vector2d {
    type Output = Vector2d;

    fn sub(self, other: f64) -> Self::Output {
        Vector2d {
            xy: &self.xy - other,
        }
    }
}


impl<T> DivAssign<T> for Vector2d<T> {
    fn div_assign(&mut self, other: T) {
        self.xy /= other;
    }
}

impl Div<f64> for &Vector2d {
    type Output = Vector2d;

    fn div(self, other: f64) -> Self::Output {
        Vector2d {
            xy: &self.xy / other,
        }
    }
}

impl MulAssign<&Vector2d> for Vector2d {
    fn mul_assign(&mut self, other: &Vector2d) {
        self.xy.dot(&other.xy);
    }
}

impl MulAssign<f64> for Vector2d {
    fn mul_assign(&mut self, other: f64) {
        self.xy *= other;
    }
}

impl Mul<&Vector2d> for &Vector2d {
    type Output = f64;

    fn mul(self, other: &Vector2d) -> Self::Output {
        self.xy.dot(&other.xy)
    }
}

impl Mul<f64> for &Vector2d {
    type Output = Vector2d;

    fn mul(self, other: f64) -> Self::Output {
        Vector2d {
            xy: &self.xy * other,
        }
    }
}

impl Mul<&Vector2d> for f64 {
    type Output = Vector2d;

    fn mul(self, other: &Vector2d) -> Self::Output {
        Vector2d {
            xy: &other.xy * self,
        }
    }
}

