use crate::vec3::*;
use crate::material::*;
use crate::hittable::{HitRecord, Hittable};
use std::sync::Arc;

pub struct Disk {
    center: Point3,
    normal: Vec3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Disk {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            normal: Vec3::new(0.0, 1.0, 0.0), // Normal vers le haut
            radius,
            mat,
        }
    }
}

impl Hittable for Disk {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = dot(self.normal, ray.direction());

        // Vérifier si le rayon est parallèle au disque
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = dot(self.center - ray.origin(), self.normal) / denom;
        if t < t_min || t > t_max {
            return None;
        }

        let p = ray.at(t);

        // Vérifier si le point est dans le rayon du disque
        if (p - self.center).length_squared() > self.radius * self.radius {
            return None;
        }

        let mut rec = HitRecord {
            t,
            p,
            mat: self.mat.clone(),
            normal: self.normal,
            front_face: true,
        };
        rec.set_face_normal(ray, self.normal);

        Some(rec)
    }
}
