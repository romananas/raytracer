use crate::vec3::*;
use crate::material::*;
use std::sync::Arc;
use crate::hittable::Hittable;
use crate::HittableList;
use crate::quad::Quad;

/// A cube represented as a collection of six `Quad` faces.
/// This struct implements the `Hittable` trait, allowing it to be used in ray tracing.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use crate::vec3::Point3;
/// use crate::material::Lambertian;
/// use crate::Cube;
///
/// let material = Arc::new(Lambertian::new(crate::vec3::Color::new(0.8, 0.3, 0.3)));
/// let cube = Cube::new(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0), material);
/// ```
pub struct Cube {
    /// A `HittableList` containing the six faces of the cube.
    pub sides: HittableList,
}

impl Cube {
    /// Creates a new cube defined by its minimum and maximum corner points.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum corner (smallest x, y, z).
    /// * `max` - The maximum corner (largest x, y, z).
    /// * `mat` - A shared reference to the material applied to all faces.
    ///
    /// # Returns
    ///
    /// A new `Cube` instance.
    ///
    /// # Example
    ///
    /// ```
    /// let min = Point3::new(0.0, 0.0, 0.0);
    /// let max = Point3::new(1.0, 1.0, 1.0);
    /// let cube = Cube::new(min, max, Arc::new(SomeMaterial));
    /// ```
    pub fn new(min: Point3, max: Point3, mat: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        // Front face
        sides.add(Box::new(Quad::new(min, dx, dy, mat.clone())));
        // Back face
        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()), dx, dy, mat.clone(),
        )));
        // Left face
        sides.add(Box::new(Quad::new(min, dz, dy, mat.clone())));
        // Right face
        sides.add(Box::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()), dz, dy, mat.clone(),
        )));
        // Bottom face
        sides.add(Box::new(Quad::new(min, dx, dz, mat.clone())));
        // Top face
        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), max.y(), min.z()), dx, dz, mat.clone(),
        )));

        Cube { sides }
    }

    /// Creates a cube from its center and size.
    ///
    /// # Arguments
    ///
    /// * `center` - The center point of the cube.
    /// * `size` - The length of each side of the cube.
    /// * `mat` - A shared reference to the material applied to all faces.
    ///
    /// # Returns
    ///
    /// A new `Cube` instance.
    ///
    /// # Example
    ///
    /// ```
    /// let center = Point3::new(0.0, 0.0, 0.0);
    /// let size = 2.0;
    /// let cube = Cube::from_center(center, size, Arc::new(SomeMaterial));
    /// ```
    pub fn from_center(center: Point3, size: f64, mat: Arc<dyn Material>) -> Self {
        let half_size = size / 2.0;
        let min = Point3::new(center.x() - half_size, center.y() - half_size, center.z() - half_size);
        let max = Point3::new(center.x() + half_size, center.y() + half_size, center.z() + half_size);
        Self::new(min, max, mat)
    }
}

impl Hittable for Cube {
    /// Checks if a given ray intersects with the cube.
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
