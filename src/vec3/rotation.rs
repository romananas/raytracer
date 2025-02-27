use super::{Point3, Vec3};

pub trait Rotate {
    fn rotate(&mut self,axis: Vec3, angle: f64) {}
    fn rotate_around(&mut self,p: Point3,axis: Vec3, angle: f64) {}
}