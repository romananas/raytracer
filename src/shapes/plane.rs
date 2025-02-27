use crate::{HitRecord, Hittable};
use crate::Ray;
use crate::vec3::{Vec3, Point3,dot,unit_vector};
 
pub struct Plane {
    pub point: Point3,   // Un point sur le plan
    pub normal: Vec3,    // Normale du plan (doit être normalisée)
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3) -> Plane {
        Plane {
            point,
            normal: unit_vector(normal), // Assure que la normale est normalisée
        }
    }
}


impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let denom = dot(self.normal, ray.direction());
        if denom.abs() < 1e-6 {
            return false; // Rayon parallèle au plan
        }

        let t = dot(self.point - ray.origin(), self.normal) / denom;
        if t < t_min || t > t_max {
            return false; // Intersection hors des limites
        }

        rec.t = t;
        rec.p = ray.at(t);
        rec.normal = self.normal; // La normale est constante pour un plan
        rec.front_face = dot(ray.direction(), self.normal) < 0.0;
        
        true
    }
}
