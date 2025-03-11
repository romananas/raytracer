use crate::vec3::*;
use crate::material::*;
use crate::hittable::{HitRecord, Hittable};
use std::sync::Arc;

/// A disk-shaped hittable object in 3D space.
/// The disk is defined by its center, radius, and a material.
/// By default, the normal is set to point upwards along the Y-axis.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use crate::vec3::Point3;
/// use crate::material::Lambertian;
/// use crate::disk::Disk;
///
/// let material = Arc::new(Lambertian::new(crate::vec3::Color::new(0.8, 0.3, 0.3)));
/// let disk = Disk::new(Point3::new(0.0, 0.0, 0.0), 1.0, material);
/// ```
pub struct Disk {
    /// The center position of the disk.
    center: Point3,
    /// The normal vector of the disk (defaults to pointing upwards).
    normal: Vec3,
    /// The radius of the disk.
    radius: f64,
    /// The material of the disk.
    mat: Arc<dyn Material>,
}

impl Disk {
    /// Creates a new disk with a given center, radius, and material.
    ///
    /// # Arguments
    ///
    /// * `center` - The center of the disk.
    /// * `radius` - The radius of the disk.
    /// * `mat` - A shared reference to the material applied to the disk.
    ///
    /// # Returns
    ///
    /// A new `Disk` instance.
    ///
    /// # Example
    ///
    /// ```
    /// let disk = Disk::new(Point3::new(0.0, 0.0, 0.0), 1.0, Arc::new(SomeMaterial));
    /// ```
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            normal: Vec3::new(0.0, 1.0, 0.0), // Default normal pointing up
            radius,
            mat,
        }
    }
}

impl Hittable for Disk {
    /// Checks if a given ray intersects with the disk.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to test for intersection.
    /// * `t_min` - The minimum valid intersection distance.
    /// * `t_max` - The maximum valid intersection distance.
    ///
    /// # Returns
    ///
    /// An `Option<HitRecord>` containing intersection data if a hit occurs.
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = dot(self.normal, ray.direction());

        // Check if the ray is parallel to the disk
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = dot(self.center - ray.origin(), self.normal) / denom;
        if t < t_min || t > t_max {
            return None;
        }

        let p = ray.at(t);

        // Check if the intersection point is within the disk's radius
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
