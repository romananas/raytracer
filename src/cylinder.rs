use crate::vec3::*;
use crate::material::*;
use std::sync::Arc;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::quad::Quad;
use crate::disk::Disk;

pub struct Cylinder {
    pub sides: HittableList,
}

impl Cylinder {
    pub fn new(center: Point3, radius: f64, height: f64, mat: Arc<dyn Material>, segments: usize) -> Self {
        let mut sides = HittableList::new();
        let half_height = height / 2.0;

        let top_center = Point3::new(center.x(), center.y() + half_height, center.z());
        let bottom_center = Point3::new(center.x(), center.y() - half_height, center.z());

        // Ajouter les bases circulaires
        sides.add(Box::new(Disk::new(top_center, radius, mat.clone())));
        sides.add(Box::new(Disk::new(bottom_center, radius, mat.clone())));

        // Ajouter la surface latérale avec des Quads
        let angle_step = std::f64::consts::TAU / segments as f64;
        for i in 0..segments {
            let theta1 = i as f64 * angle_step;
            let theta2 = (i + 1) as f64 * angle_step;

            let x1 = radius * theta1.cos();
            let z1 = radius * theta1.sin();
            let x2 = radius * theta2.cos();
            let z2 = radius * theta2.sin();

            let p1 = Point3::new(center.x() + x1, bottom_center.y(), center.z() + z1);
            let p2 = Point3::new(center.x() + x2, bottom_center.y(), center.z() + z2);
            // let p3 = Point3::new(center.x() + x2, top_center.y(), center.z() + z2);
            let p4 = Point3::new(center.x() + x1, top_center.y(), center.z() + z1);

            sides.add(Box::new(Quad::new(p1, p2 - p1, p4 - p1, mat.clone())));
        }

        Cylinder { sides }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }
}
