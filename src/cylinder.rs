use crate::vec3::*;
use crate::material::*;
use std::sync::Arc;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::quad::Quad;
use crate::disk::Disk;

/// A cylinder shape composed of multiple quads and two disks.
/// The cylinder is centered around a given point, with a specified radius, height, and material.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use crate::vec3::Point3;
/// use crate::material::Lambertian;
/// use crate::cylinder::Cylinder;
///
/// let material = Arc::new(Lambertian::new(crate::vec3::Color::new(0.8, 0.3, 0.3)));
/// let cylinder = Cylinder::new(Point3::new(0.0, 0.0, 0.0), 1.0, 2.0, material, 16);
/// ```
pub struct Cylinder {
    /// The list of hittable objects that make up the cylinder (quads for the sides and disks for the top and bottom).
    pub sides: HittableList,
}

impl Cylinder {
    /// Creates a new cylinder with a given center, radius, height, material, and number of segments.
    ///
    /// The cylinder is approximated using quads for the sides and disks for the top and bottom.
    ///
    /// # Arguments
    ///
    /// * `center` - The center of the cylinder.
    /// * `radius` - The radius of the cylinder.
    /// * `height` - The height of the cylinder.
    /// * `mat` - A shared reference to the material applied to the cylinder.
    /// * `segments` - The number of segments used to approximate the curved surface.
    ///
    /// # Returns
    ///
    /// A new `Cylinder` instance.
    ///
    /// # Example
    ///
    /// ```
    /// let cylinder = Cylinder::new(Point3::new(0.0, 0.0, 0.0), 1.0, 2.0, Arc::new(SomeMaterial), 16);
    /// ```
    pub fn new(center: Point3, radius: f64, height: f64, mat: Arc<dyn Material>, segments: usize) -> Self {
        let mut sides = HittableList::new();
        let half_height = height / 2.0;

        let top_center = Point3::new(center.x(), center.y() + half_height, center.z());
        let bottom_center = Point3::new(center.x(), center.y() - half_height, center.z());

        // Add circular base disks
        sides.add(Box::new(Disk::new(top_center, radius, mat.clone())));
        sides.add(Box::new(Disk::new(bottom_center, radius, mat.clone())));

        // Add lateral surface quads
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
            let p4 = Point3::new(center.x() + x1, top_center.y(), center.z() + z1);

            sides.add(Box::new(Quad::new(p1, p2 - p1, p4 - p1, mat.clone())));
        }

        Cylinder { sides }
    }
}

impl Hittable for Cylinder {
    /// Checks if a given ray intersects with any of the cylinder's components (sides or bases).
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
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }
}
