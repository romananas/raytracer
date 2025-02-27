use crate::{HitRecord, Hittable};
use crate::Ray;
use crate::vec3::*;

pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    normal: Vec3,
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3) -> Self {
        let normal = unit_vector(cross(v1 - v0, v2 - v0));
        Self { v0, v1, v2, normal }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = cross(ray.direction(), edge2);
        let a = dot(edge1, h);

        if a.abs() < 1e-8 {
            return false; // Rayon parallèle au triangle
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.v0;
        let u = f * dot(s, h);

        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = cross(s, edge1);
        let v = f * dot(ray.direction(), q);

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = f * dot(edge2, q);
        if t < t_min || t > t_max {
            return false;
        }

        rec.t = t;
        rec.p = ray.at(t);
        rec.normal = self.normal;
        true
    }
}