use crate::traits::FloatWithConst;
use crate::xy::XY;

#[derive(Debug, Clone, Copy)]
pub struct Vector2d<T = f64> {
    pub xy: XY<T>,
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
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Vector2d { xy: XY::new() }
    }

    pub fn from_xy(xy: XY<T>) -> Self {
        Vector2d { xy }
    }

    pub fn from_coords(x: T, y: T) -> Self {
        Vector2d {
            xy: XY::from_coords(x, y),
        }
    }

    pub fn get_coords(&self) -> (T, T) {
        (self.xy.x, self.xy.y)
    }

    pub fn set_coords(&mut self, x: T, y: T) {
        self.xy.x = x;
        self.xy.y = y;
    }

    pub fn get_x(&self) -> T {
        self.xy.x
    }

    pub fn set_x(&mut self, x: T) {
        self.xy.x = x;
    }

    pub fn get_y(&self) -> T {
        self.xy.y
    }

    pub fn set_y(&mut self, y: T) {
        self.xy.y = y;
    }

    pub fn get_xy(&self) -> &XY<T> {
        &self.xy
    }

    pub fn set_xy(&mut self, xy: XY<T>) {
        self.xy = xy;
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
            let ang = self.angle(other)?.abs();
            return Ok(is_equal_length && ang <= angular_tolerance);
        }
        Ok(is_equal_length)
    }
}

impl<T> From<(T, T)> for Vector2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn from(coords: (T, T)) -> Self {
        Vector2d::from_coords(coords.0, coords.1)
    }
}

impl<T> From<[T; 2]> for Vector2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn from(coords: [T; 2]) -> Self {
        Vector2d::from_coords(coords[0], coords[1])
    }
}

impl<T> Vector2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn length(&self) -> T {
        self.xy.length()
    }

    pub fn squared_length(&self) -> T {
        self.xy.squared_length()
    }

    pub fn angle(&self, other: &Self) -> Result<T, &'static str> {
        let mag = self.length();
        let other_mag = other.length();
        if mag <= T::min_positive_value() || other_mag <= T::min_positive_value() {
            return Err("Cannot calculate angle to zero vector");
        }
        let d = mag * other_mag;
        let cosinus = self.xy.dot(&other.get_xy()) / d;
        let sinus = self.xy.cross(&other.get_xy()) / d;

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

    pub fn is_orthogonal(&self, other: &Self, ang_tolerance: T) -> Result<bool, &'static str> {
        let ang = (T::frac_pi_2() - self.angle(other)?.abs()).abs();
        if ang < ang_tolerance {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn is_opposite(&self, other: &Self, ang_tolerance: T) -> Result<bool, &'static str> {
        let ang = self.angle(other)?.abs();
        if (T::pi() - ang).abs() < ang_tolerance {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn is_parallel(&self, other: &Self, ang_tolerance: T) -> Result<bool, &'static str> {
        let ang = self.angle(other)?.abs();
        if (T::pi() - ang).abs() < ang_tolerance || ang < ang_tolerance {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
    
}
