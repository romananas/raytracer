use crate::vec3::{Point3, Vec3};
 
/// A `Ray` represents a ray in 3D space, defined by an origin and a direction.
///
/// # Fields
/// 
/// * `orig` - The origin point of the ray.
/// * `dir` - The direction vector of the ray.
///
/// # Methods
///
/// * `new` - Constructs a new `Ray` with the given origin and direction.
/// * `origin` - Returns the origin point of the ray.
/// * `direction` - Returns the direction vector of the ray.
/// * `at` - Computes the point along the ray at a given distance `t`.
#[derive(Default,Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}
 
impl Ray {
    /// Construct for ray
    /// 
    /// # Exemple
    /// 
    /// ```
    /// use raytracer::{Point3, Vec3};
    /// use raytracer::Ray;
    /// 
    /// let origin = Point3::new(0.0, 0.0, 0.0);
    /// let direction = Vec3::new(1.0, 0.0, 0.0);
    /// let ray = Ray::new(origin, direction);
    /// ``` 
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
    
    /// Give the orig field value
    pub fn origin(&self) -> Point3 {
        self.orig
    }
 
    /// Give the orig dir value
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
 
    /// Calculate where the ray go
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}