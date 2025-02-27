use crate::hittable::{HitRecord,Hittable};
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3,unit_vector,cross};
use crate::Triangle;

#[derive(Clone, Copy,Default)]
pub struct Quad {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    v3: Point3,
    normal: Vec3,
}

impl Quad {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, v3: Point3) -> Self {
        let normal = unit_vector(cross(v1 - v0, v3 - v0)); // Calcul de la normale
        Self { v0, v1, v2, v3, normal }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Test de l'intersection avec le premier triangle (v0, v1, v2)
        let tri1 = Triangle::new(self.v0, self.v1, self.v2);
        let tri2 = Triangle::new(self.v0, self.v2, self.v3);

        tri1.hit(ray, t_min, t_max, rec) || tri2.hit(ray, t_min, t_max, rec)
    }
}

