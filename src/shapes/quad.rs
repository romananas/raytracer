use crate::{HitRecord,Hittable};
use crate::Ray;
use crate::vec3::{cross, unit_vector, Point3, Rotate, Vec3};
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

impl Rotate for Quad {
    fn rotate(&mut self, axis: Vec3, angle: f64) {
        // Calculer le centre du quad
        let center = (self.v0 + self.v1 + self.v2 + self.v3) / 4.0;

        // Appliquer la rotation autour du centre pour chaque sommet
        self.v0.rotate_around(center, axis, angle);
        self.v1.rotate_around(center, axis, angle);
        self.v2.rotate_around(center, axis, angle);
        self.v3.rotate_around(center, axis, angle);
    }

    fn rotate_around(&mut self, p: Point3, axis: Vec3, angle: f64) {
        // Appliquer la rotation autour du point p pour chaque sommet
        self.v0.rotate_around(p, axis, angle);
        self.v1.rotate_around(p, axis, angle);
        self.v2.rotate_around(p, axis, angle);
        self.v3.rotate_around(p, axis, angle);
    }
}
