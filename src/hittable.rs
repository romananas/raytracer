use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

/// Represents a hit record for a ray-object intersection.
/// 
/// This struct stores details about where a ray intersects an object, including the intersection point,
/// surface normal, material, distance along the ray, and whether the intersection is on the front face.
/// 
/// Imagine shining a laser pointer at an object. This structure keeps track of where the
/// laser hits and how it bounces.
pub struct HitRecord {
    pub p: Point3, // Intersection point
    pub normal: Vec3, // Surface normal at intersection
    pub mat: Arc<dyn Material>, // Material of the hit object
    pub t: f64, // Distance from ray origin to intersection
    pub front_face: bool, // Whether the hit is on the front face of the object
}

impl HitRecord {
    /// Sets the surface normal and determines whether the ray hit the front or back face.
    /// 
    /// This function ensures that the normal always points against the incoming ray,
    /// which helps in shading calculations. If the ray comes from outside, the normal stays as is;
    /// if the ray is inside the object, the normal is flipped.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

/// Trait for objects that can be hit by rays.
/// 
/// Any object that can be intersected by a ray must implement this trait.
/// 
/// Think of a `Hittable` object as anything that can be "seen" by a ray,
/// such as spheres, planes, or more complex shapes.
pub trait Hittable: Send + Sync {
    /// Determines if and where a ray hits the object within a given range.
    /// 
    /// - `ray`: The incoming ray.
    /// - `t_min`: The minimum distance allowed for a valid hit.
    /// - `t_max`: The maximum distance allowed for a valid hit.
    /// - Returns `Some(HitRecord)` if there is a hit, otherwise `None`.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}