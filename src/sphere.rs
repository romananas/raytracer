use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3};

/// Represents a sphere in 3D space.
/// 
/// A sphere is defined by its center, radius, and material.
/// It implements the `Hittable` trait to determine ray intersections.
/// 
/// **For beginners:** Imagine a perfect ball in space. This code checks if a ray (a straight line) hits the ball.
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    /// Creates a new sphere with a given center, radius, and material.
    /// 
    /// **For beginners:** Think of this as defining a ball’s position, size, and texture.
    pub fn new(cen: Point3, r: f64, m: Arc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat: m,
        }
    }
}

impl Hittable for Sphere {
    /// Determines if a ray intersects the sphere.
    /// 
    /// Solves the quadratic equation for ray-sphere intersection:
    /// 
    /// `(P - C) • (P - C) = R²`, where `P = O + tD` (ray equation).
    /// 
    /// **For beginners:** This checks whether a line (ray) touches or goes through the sphere.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Default::default(),
            front_face: Default::default(),
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}