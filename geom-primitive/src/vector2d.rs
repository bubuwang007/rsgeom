use crate::point2d::Point2d;
use crate::xy::XY;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2d {
    xy: XY,
}

impl Vector2d {

    pub const VX: Vector2d = Vector2d {
        xy: XY { x: 1.0, y: 0.0 },
    };

    pub const VY: Vector2d = Vector2d {
        xy: XY { x: 0.0, y: 1.0 },
    };

    pub fn new() -> Self {
        Vector2d { xy: XY::new() }
    }

    // pub fn from_direction2d(x: f64, y: f64) -> Self {}

    pub fn from_coordinates(x: f64, y: f64) -> Self {
        Vector2d {
            xy: XY::from_coordinates(x, y),
        }
    }

    pub fn from_xy(xy: XY) -> Self {
        Vector2d { xy }
    }

    pub fn from_points(p1: &Point2d, p2: &Point2d) -> Self {
        Vector2d {
            xy: XY::from_coordinates(p2.x() - p1.x(), p2.y() - p1.y()),
        }
    }

    pub fn to_string(&self) -> String {
        format!("Vector2d({}, {})", self.xy.x, self.xy.y)
    }

    pub fn x(&self) -> f64 {
        self.xy.x
    }

    pub fn y(&self) -> f64 {
        self.xy.y
    }

    pub fn xy(&self) -> &XY {
        &self.xy
    }

    pub fn coord(&self) -> (f64, f64) {
        (self.xy.x, self.xy.y)
    }

    pub fn set_x(&mut self, x: f64) {
        self.xy.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.xy.y = y;
    }

    pub fn set_xy(&mut self, xy: XY) {
        self.xy = xy;
    }

    pub fn set_coord(&mut self, x: f64, y: f64) {
        self.xy.x = x;
        self.xy.y = y;
    }

    pub fn length(&self) -> f64 {
        self.xy.modulus()
    }

    pub fn square_length(&self) -> f64 {
        self.xy.square_modulus()
    }

    pub fn angle_to_vector(&self, other: &Self) -> Result<f64, &'static str> {
        let mag = self.length();
        let other_mag = other.length();
        if mag <= std::f64::MIN_POSITIVE || other_mag <= std::f64::MIN_POSITIVE {
            return Err("Cannot calculate angle to zero vector");
        }
        let d = mag * other_mag;
        let cosinus = self.xy.dot(&other.xy()) / d;
        let sinus = self.xy.cross(&other.xy()) / d;

        if cosinus > -0.70710678118655 && cosinus < 0.70710678118655 {
            if sinus > 0.0 {
                return Ok(cosinus.acos());
            } else {
                return Ok(-cosinus.acos());
            }
        } else {
            if cosinus > 0.0 {
                return Ok(sinus.asin());
            } else {
                if sinus > 0.0 {
                    return Ok(std::f64::consts::PI - sinus.asin());
                } else {
                    return Ok(-std::f64::consts::PI - sinus.asin());
                }
            }
        }
    }

    pub fn is_equal(
        &self,
        other: &Self,
        linear_tolerance: f64,
        angular_tolerance: f64,
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

    pub fn is_orthogonal(&self, other: &Self, ang_tolerance: f64) -> Result<bool, &'static str> {
        let ang = (std::f64::consts::FRAC_PI_2 - self.angle_to_vector(other)?.abs()).abs();
        if ang < ang_tolerance {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn is_opposite(&self, other: &Self, ang_tolerance: f64) -> Result<bool, &'static str> {
        let ang = self.angle_to_vector(other)?.abs();
        if (std::f64::consts::PI - ang).abs() < ang_tolerance {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn is_parallel(&self, other: &Self, ang_tolerance: f64) -> Result<bool, &'static str> {
        let ang = self.angle_to_vector(other)?.abs();
        if (std::f64::consts::PI - ang).abs() < ang_tolerance || ang < ang_tolerance {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn cross(&self, other: &Self) -> f64 {
        self.xy.cross(&other.xy())
    }

    pub fn cross_abs(&self, other: &Self) -> f64 {
        self.xy.cross_abs(&other.xy())
    }

    pub fn square_cross_abs(&self, other: &Self) -> f64 {
        self.xy.square_cross_abs(&other.xy())
    }

    pub fn dot(&self, other: &Self) -> f64 {
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
    pub fn set_linear_form_2(&mut self, a1: f64, v1: &Vector2d, a2: f64, v2: &Vector2d) {
        self.xy.set_linear_form_2(a1, v1.xy(), a2, v2.xy());
    }

    // a1 * v1 + v2
    pub fn set_linear_form_2a(&mut self, a1: f64, v1: &Vector2d, v2: &Vector2d) {
        self.xy.set_linear_form_2a(a1, v1.xy(), v2.xy());
    }

    // v1 + v2
    pub fn set_linear_form_2b(&mut self, v1: &Vector2d, v2: &Vector2d) {
        self.xy.set_linear_form_2b(v1.xy(), v2.xy());
    }

    // a1 * v1 + a2 * v2 + v3
    pub fn set_linear_form_3(&mut self, a1: f64, v1: &Vector2d, a2: f64, v2: &Vector2d, v3: &Vector2d) {
        self.xy.set_linear_form_3(a1, v1.xy(), a2, v2.xy(), v3.xy());
    }

    // pub fn mirror_by_ax2d()

    // 存在疑问，结果永远是other
    pub fn mirror_by_vector2d(&mut self, other: &Self) {
        let d = other.xy().modulus();
        if d > std::f64::MIN_POSITIVE {
            let x = other.x();
            let y = other.y();
            let a = x / d;
            let b = y / d;
            let m1 = 2.0 * a * b;
            self.set_x(((2.0 * a * a) - 1.0) * x + m1 * y);
            self.set_y(m1 * x + ((2.0 * b * b - 1.0) * y));
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

    pub fn scale(&mut self, scale: f64) {
        *self *= scale;
    }

    pub fn scale_new(&self, scale: f64) -> Self {
        let mut result = *self;
        result.scale(scale);
        result
    }

    // transform

}

use std::ops::{Index, IndexMut};

impl Index<usize> for Vector2d {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.xy.x,
            1 => &self.xy.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector2d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.xy.x,
            1 => &mut self.xy.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Neg for &Vector2d {
    type Output = Vector2d;

    fn neg(self) -> Self::Output {
        Vector2d { xy: -&self.xy }
    }
}

impl AddAssign<&Vector2d> for Vector2d {
    fn add_assign(&mut self, other: &Vector2d) {
        self.xy += &other.xy;
    }
}

impl AddAssign<f64> for Vector2d {
    fn add_assign(&mut self, other: f64) {
        self.xy += other;
    }
}

impl Add<&Vector2d> for &Vector2d {
    type Output = Vector2d;

    fn add(self, other: &Vector2d) -> Self::Output {
        Vector2d {
            xy: &self.xy + &other.xy,
        }
    }
}

impl Add<f64> for &Vector2d {
    type Output = Vector2d;

    fn add(self, other: f64) -> Self::Output {
        Vector2d {
            xy: self.xy + other,
        }
    }
}

impl DivAssign<f64> for Vector2d {
    fn div_assign(&mut self, other: f64) {
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

impl SubAssign<&Vector2d> for Vector2d {
    fn sub_assign(&mut self, other: &Vector2d) {
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
