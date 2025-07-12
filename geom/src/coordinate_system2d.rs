use crate::direction2d::Direction2d;
use crate::point2d::Point2d;
use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct CoordinateSystem2d<T = f64> {
    pub origin: Point2d<T>,
    pub vydir: Direction2d<T>,
    pub vxdir: Direction2d<T>,
}

impl<T> std::fmt::Display for CoordinateSystem2d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CoordinateSystem2d(origin: {}, vydir: {}, vxdir: {})",
            self.origin, self.vydir, self.vxdir
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
            vydir: Direction2d::new(),
            vxdir: Direction2d::new(),
        }
    }

    pub fn from_origin_vydir_vxdir<P, VY, VX>(origin: P, vydir: VY, vxdir: VX) -> Self
    where
        P: Into<Point2d<T>>,
        VY: Into<Direction2d<T>>,
        VX: Into<Direction2d<T>>,
    {
        CoordinateSystem2d {
            origin: origin.into(),
            vydir: vydir.into(),
            vxdir: vxdir.into(),
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

    pub fn get_vydir(&self) -> &Direction2d<T> {
        &self.vydir
    }

    pub fn set_vydir<D>(&mut self, vydir: D)
    where
        D: Into<Direction2d<T>>,
    {
        self.vydir = vydir.into();
    }

    pub fn get_vxdir(&self) -> &Direction2d<T> {
        &self.vxdir
    }

    pub fn set_vxdir<D>(&mut self, vxdir: D)
    where
        D: Into<Direction2d<T>>,
    {
        self.vxdir = vxdir.into();
    }
}
