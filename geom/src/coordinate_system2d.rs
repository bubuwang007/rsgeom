use crate::direction2d::Direction2d;
use crate::point2d::Point2d;
use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct CoordinateSystem2d<T = f64> {
    pub origin: Point2d<T>,
    pub vdir: Direction2d<T>,
}

impl<T> std::fmt::Display for CoordinateSystem2d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CoordinateSystem2d(origin: {}, vdir: {})",
            self.origin, self.vdir
        )
    }
}

impl<T> CoordinateSystem2d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        CoordinateSystem2d {
            origin: Point2d::new(),
            vdir: Direction2d::new(),
        }
    }

    pub fn from_origin_vydir_vxdir<P, V>(origin: P, vdir: V) -> Self
    where
        P: Into<Point2d<T>>,
        V: Into<Direction2d<T>>,
    {
        CoordinateSystem2d {
            origin: origin.into(),
            vdir: vdir.into(),
        }
    }

    pub fn get_origin(&self) -> &Point2d<T> {
        &self.origin
    }

    pub fn set_origin<P>(&mut self, origin: P)
    where
        P: Into<Point2d<T>>,
    {
        self.origin = origin.into();
    }

    pub fn get_dir(&self) -> &Direction2d<T> {
        &self.vdir
    }

    pub fn set_dir<D>(&mut self, vydir: D)
    where
        D: Into<Direction2d<T>>,
    {
        self.vdir = vydir.into();
    }

}
