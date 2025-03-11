use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
 
/// A list of objects that implement the `Hittable` trait.
/// This struct is useful for managing multiple hittable objects in a ray tracing scene.
///
/// # Example
///
/// ```
/// use crate::hittable::HittableList;
/// use crate::sphere::Sphere;
/// use crate::vec3::Point3;
/// use std::sync::Arc;
/// use crate::material::Lambertian;
///
/// let mut world = HittableList::new();
/// let material = Arc::new(Lambertian::new(crate::vec3::Color::new(0.8, 0.3, 0.3)));
/// world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material)));
/// ```
#[derive(Default)]
pub struct HittableList {
    /// A vector storing objects that implement the `Hittable` trait.
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    /// Creates a new, empty `HittableList`.
    ///
    /// # Returns
    ///
    /// A new `HittableList` instance.
    ///
    /// # Example
    ///
    /// ```
    /// let list = HittableList::new();
    /// ```
    pub fn new() -> HittableList {
        Default::default()
    }

    /// Adds a new hittable object to the list.
    ///
    /// # Arguments
    ///
    /// * `object` - A boxed object that implements the `Hittable` trait.
    ///
    /// # Example
    ///
    /// ```
    /// let mut list = HittableList::new();
    /// list.add(Box::new(SomeHittableObject));
    /// ```
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    /// Determines if a ray hits any object in the list.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to test for intersection.
    /// * `t_min` - The minimum valid intersection distance.
    /// * `t_max` - The maximum valid intersection distance.
    ///
    /// # Returns
    ///
    /// An `Option<HitRecord>` containing the closest intersection, if any.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
