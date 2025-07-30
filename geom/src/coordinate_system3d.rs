use crate::Axis3d;
use crate::Direction3d;
use crate::Point3d;
use crate::traits::FloatWithConst;

#[derive(Debug, Clone, Copy)]
pub struct CoordinateSystem3d<T = f64> {
    pub axis: Axis3d<T>,
    pub vydir: Direction3d<T>,
    pub vxdir: Direction3d<T>,
}

impl<T> std::fmt::Display for CoordinateSystem3d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CoordinateSystem3d(axis: {}, vydir: {}, vxdir: {})",
            self.axis, self.vydir, self.vxdir
        )
    }
}

impl<T> CoordinateSystem3d<T>
where
    T: Copy + Default + FloatWithConst,
{
    pub fn new() -> Self {
        CoordinateSystem3d {
            axis: Axis3d::new(),
            vydir: Direction3d::new(),
            vxdir: Direction3d::new(),
        }
    }

    pub fn from_axis_vydir_vxdir<A, VY, VX>(axis: A, vydir: VY, vxdir: VX) -> Self
    where
        A: Into<Axis3d<T>>,
        VY: Into<Direction3d<T>>,
        VX: Into<Direction3d<T>>,
    {
        let coord = CoordinateSystem3d {
            axis: axis.into(),
            vydir: vydir.into(),
            vxdir: vxdir.into(),
        };
        // coord.vxdir

        coord
    }

    pub fn get_origin(&self) -> &Point3d<T> {
        &self.axis.location
    }

    pub fn set_origin<P>(&mut self, origin: P)
    where
        P: Into<Point3d<T>>,
    {
        self.axis.location = origin.into();
    }

    pub fn get_axis(&self) -> &Axis3d<T> {
        &self.axis
    }

    pub fn set_axis<A>(&mut self, axis: A)
    where
        A: Into<Axis3d<T>>,
    {
        self.axis = axis.into();
    }

    pub fn get_vydir(&self) -> &Direction3d<T> {
        &self.vydir
    }

    pub fn set_vydir<D>(&mut self, vydir: D)
    where
        D: Into<Direction3d<T>>,
    {
        self.vydir = vydir.into();
    }

    pub fn get_vxdir(&self) -> &Direction3d<T> {
        &self.vxdir
    }

    pub fn set_vxdir<D>(&mut self, vxdir: D)
    where
        D: Into<Direction3d<T>>,
    {
        self.vxdir = vxdir.into();
    }
}
