use crate::vec3::*;
use crate::material::*;
use std::sync::Arc;
use crate::hittable::Hittable;
use crate::HittableList;
use crate::quad::Quad;

pub struct Cube {
    pub sides: HittableList,
}

impl Cube {
    pub fn new(min: Point3, max: Point3, mat: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        // Face avant
        sides.add(Box::new(Quad::new(min, dx, dy, mat.clone())));
        // Face arrière
        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()), dx, dy, mat.clone(),
        )));
        // Face gauche
        sides.add(Box::new(Quad::new(min, dz, dy, mat.clone())));
        // Face droite
        sides.add(Box::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()), dz, dy, mat.clone(),
        )));
        // Face du bas
        sides.add(Box::new(Quad::new(min, dx, dz, mat.clone())));
        // Face du haut
        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), max.y(), min.z()), dx, dz, mat.clone(),
        )));

        Cube { sides }
    }

    pub fn from_center(center: Point3, size: f64, mat: Arc<dyn Material>) -> Self {
        let half_size = size / 2.0;
        let min = Point3::new(center.x() - half_size, center.y() - half_size, center.z() - half_size);
        let max = Point3::new(center.x() + half_size, center.y() + half_size, center.z() + half_size);
        Self::new(min, max, mat)
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }
}
