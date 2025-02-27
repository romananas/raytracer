use super::quad::Quad;
use crate::Hittable;
use crate::Point3;

#[derive(Clone,Default)]
pub struct Cube {
    quads: Vec<Quad>,
}

impl Cube {
    pub fn new(center: Point3, size: f64) -> Self {
        let s = size / 2.0;

        let front = Quad::new(
            Point3::new(center.x() - s, center.y() - s, center.z() + s),
            Point3::new(center.x() + s, center.y() - s, center.z() + s),
            Point3::new(center.x() + s, center.y() + s, center.z() + s),
            Point3::new(center.x() - s, center.y() + s, center.z() + s),
        );

        let back = Quad::new(
            Point3::new(center.x() - s, center.y() - s, center.z() - s),
            Point3::new(center.x() + s, center.y() - s, center.z() - s),
            Point3::new(center.x() + s, center.y() + s, center.z() - s),
            Point3::new(center.x() - s, center.y() + s, center.z() - s),
        );

        let left = Quad::new(
            Point3::new(center.x() - s, center.y() - s, center.z() - s),
            Point3::new(center.x() - s, center.y() - s, center.z() + s),
            Point3::new(center.x() - s, center.y() + s, center.z() + s),
            Point3::new(center.x() - s, center.y() + s, center.z() - s),
        );

        let right = Quad::new(
            Point3::new(center.x() + s, center.y() - s, center.z() - s),
            Point3::new(center.x() + s, center.y() - s, center.z() + s),
            Point3::new(center.x() + s, center.y() + s, center.z() + s),
            Point3::new(center.x() + s, center.y() + s, center.z() - s),
        );

        let top = Quad::new(
            Point3::new(center.x() - s, center.y() + s, center.z() - s),
            Point3::new(center.x() + s, center.y() + s, center.z() - s),
            Point3::new(center.x() + s, center.y() + s, center.z() + s),
            Point3::new(center.x() - s, center.y() + s, center.z() + s),
        );

        let bottom = Quad::new(
            Point3::new(center.x() - s, center.y() - s, center.z() - s),
            Point3::new(center.x() + s, center.y() - s, center.z() - s),
            Point3::new(center.x() + s, center.y() - s, center.z() + s),
            Point3::new(center.x() - s, center.y() - s, center.z() + s),
        );

        Self {
            quads: vec![front, back, left, right, top, bottom],
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut crate::hittable::HitRecord) -> bool {
        for side in self.quads.clone() {
            if side.hit(ray, t_min, t_max, rec) {
                return true;
            }
        }
        false
    }
}