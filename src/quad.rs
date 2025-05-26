use crate::vec3::*;
use crate::material::*;
use std::sync::Arc;
use crate::hittable::{HitRecord,Hittable};

/// Represents a quadrilateral (quad) in 3D space defined by a point `q`
/// and two edge vectors `u` and `v`.
///
/// The quad lies on a plane, with `q` as one corner, `u` and `v` defining its sides,
/// and a shared material reference via `Arc<dyn Material>`.
pub struct Quad {
    /// Origin point of the quad (one of its corners).
    q: Point3,
    /// One directional vector defining a side of the quad.
    v: Vec3,
    /// Another directional vector defining the adjacent side of the quad.
    u: Vec3,
    /// Helper vector used for barycentric coordinate calculation.
    w: Vec3,
    /// Unit normal vector of the plane the quad lies in.
    normal: Vec3,
    /// Plane distance from origin used in the plane equation.
    d: f64,
    /// Shared reference to the material applied to the quad.
    mat: Arc<dyn Material>,
}

impl Quad {
    /// Constructs a new `Quad` from an origin point `q` and two directional vectors `u` and `v`.
    ///
    /// Automatically computes the normal of the plane and internal parameters for ray intersection.
    ///
    /// # Arguments
    /// * `q` - A point representing one corner of the quad.
    /// * `u` - A vector defining one edge of the quad.
    /// * `v` - A vector defining the adjacent edge of the quad.
    /// * `mat` - The material to apply to the quad.
    ///
    /// # Returns
    /// A new instance of `Quad`.
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = cross(u, v);
        let normal = unit_vector(n);
        let d = dot(normal, q);
        let w = n / dot(n, n);

        Quad {
            q,
            u,
            v,
            w,
            mat,
            d,
            normal,
        }
    }
}

impl Hittable for Quad {
    /// Tests whether a given ray hits the quad.
    ///
    /// First computes intersection with the plane, then checks if the hit point
    /// lies within the boundaries of the quad using barycentric coordinates.
    ///
    /// # Arguments
    /// * `ray` - The ray to test for intersection.
    /// * `t_min` - Minimum valid distance for a hit.
    /// * `t_max` - Maximum valid distance for a hit.
    ///
    /// # Returns
    /// * `Some(HitRecord)` if the ray hits the quad.
    /// * `None` if there's no valid intersection.
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = dot(self.normal, ray.direction());

        // If the ray is parallel to the quad's plane
        if f64::abs(denom) < 1e-8 {
            return None;
        }

        let t = (self.d - dot(self.normal, ray.origin())) / denom;
        if t < t_min || t > t_max {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hit_point = intersection - self.q;

        // Barycentric coordinate checks
        let alpha = dot(self.w, cross(planar_hit_point, self.v));
        let beta = dot(self.w, cross(self.u, planar_hit_point));

        // Check if the hit point lies inside the quad
        if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 {
            return None;
        }

        let mut rec = HitRecord {
            t,
            p: intersection,
            mat: self.mat.clone(),
            normal: Default::default(),
            front_face: Default::default(),
        };

        rec.set_face_normal(ray, self.normal);

        Some(rec)
    }
}
