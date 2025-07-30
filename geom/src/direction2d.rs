use crate::traits::FloatWithConst;
use crate::XY;

#[derive(Debug, Clone, Copy)]
pub struct Direction2d<T = f64> {
    pub xy: XY<T>,
}

impl<T> std::fmt::Display for Direction2d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Direction2d({}, {})", self.xy.x, self.xy.y)
    }
}

impl<T> Direction2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        Direction2d::default()
    }

    pub fn from_xy<X>(xy: X) -> Self
    where
        X: Into<XY<T>>,
    {
        let mut d: Direction2d<T> = Direction2d { xy: xy.into() };
        d.normalize();
        d
    }

    pub fn from_coords(x: T, y: T) -> Self {
        let mut d = Direction2d {
            xy: XY::from_coords(x, y),
        };
        d.normalize();
        d
    }

    #[inline]
    fn normalize(&mut self) {
        self.xy.normalize();
    }

    pub fn get_coords(&self) -> (T, T) {
        (self.xy.x, self.xy.y)
    }

    pub fn set_coords(&mut self, x: T, y: T) {
        self.xy.x = x;
        self.xy.y = y;
        self.normalize();
    }

    pub fn get_x(&self) -> T {
        self.xy.x
    }

    pub fn get_y(&self) -> T {
        self.xy.y
    }

    pub fn get_xy(&self) -> &XY<T> {
        &self.xy
    }

    pub fn set_xy(&mut self, xy: XY<T>) {
        self.xy = xy;
        self.normalize();
    }

    pub fn is_equal(&self, other: &Self, tolerance: T) -> bool {
        self.xy.is_equal(&other.xy, tolerance)
    }
}

impl<T> From<(T, T)> for Direction2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn from(coords: (T, T)) -> Self {
        Direction2d::from_coords(coords.0, coords.1)
    }
}

impl<T> Default for Direction2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    fn default() -> Self {
        Direction2d {
            xy: XY::from_coords(T::from(1.0).unwrap(), T::from(0.0).unwrap()),
        }
    }
}

impl<T> Direction2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn angle(&self, other: &Self) -> T {
        let cosinus = self.xy.dot(&other.xy);
        let sinus = self.xy.cross(&other.xy);

        let zero = T::from(0.0).unwrap();
        if cosinus > T::from(-0.70710678118655).unwrap()
            && cosinus < T::from(0.70710678118655).unwrap()
        {
            if sinus > zero {
                return cosinus.acos();
            } else {
                return -cosinus.acos();
            }
        } else {
            if cosinus > zero {
                return sinus.asin();
            } else {
                if sinus > zero {
                    return T::pi() - sinus.asin();
                } else {
                    return -T::pi() - sinus.asin();
                }
            }
        }
    }

    pub fn is_orthogonal(&self, other: &Self, tolerance: T) -> bool {
        let ang = (T::frac_pi_2() - self.angle(other).abs()).abs();
        ang <= tolerance
    }

    pub fn is_opposite(&self, other: &Self, tolerance: T) -> bool {
        let ang = T::pi() - self.angle(other).abs();
        ang <= tolerance
    }

    pub fn is_parallel(&self, other: &Self, tolerance: T) -> bool {
        let ang = self.angle(other).abs();
        ang <= tolerance || (T::pi() - ang) <= tolerance
    }

    pub fn cross(&self, other: &Self) -> T {
        self.xy.cross(&other.xy)
    }

    pub fn dot(&self, other: &Self) -> T {
        self.xy.dot(&other.xy)
    }

    pub fn reverse(&mut self) {
        self.xy.reverse();
    }

    pub fn reverse_new(&self) -> Self {
        let mut new_dir = *self;
        new_dir.reverse();
        new_dir
    }
}
