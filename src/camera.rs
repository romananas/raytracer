use crate::common;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
 
/// A camera in a 3D space, used for ray tracing.
///
/// # Fields
/// - `origin`: The origin point of the camera.
/// - `lower_left_corner`: The lower left corner of the viewport.
/// - `horizontal`: The horizontal vector of the viewport.
/// - `vertical`: The vertical vector of the viewport.
/// - `u`: The camera's horizontal axis in world space.
/// - `v`: The camera's vertical axis in world space.
/// - `lens_radius`: The radius of the camera's lens.
///
/// # Methods
/// 
/// ## `new`
/// Creates a new `Camera` instance.
/// 
/// ### Parameters
/// - `lookfrom`: The point where the camera is located.
/// - `lookat`: The point where the camera is looking at.
/// - `vup`: The "up" direction vector.
/// - `vfov`: Vertical field-of-view in degrees.
/// - `aspect_ratio`: The aspect ratio of the viewport.
/// - `aperture`: The aperture size of the camera.
/// - `focus_dist`: The focus distance of the camera.
/// 
/// ### Returns
/// A new `Camera` instance.
///
/// ## `get_ray`
/// Generates a ray from the camera through the viewport at coordinates (s, t).
/// 
/// ### Parameters
/// - `s`: The horizontal coordinate on the viewport (0.0 to 1.0).
/// - `t`: The vertical coordinate on the viewport (0.0 to 1.0).
/// 
/// ### Returns
/// A `Ray` instance originating from the camera and passing through the viewport at (s, t).
#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    /// Constructor for a Camera
    /// 
    /// # Exemple
    /// 
    /// ```
    ///  use raytracer::{Point3, Vec3};
    ///  use raytracer::Camera;
    /// 
    ///  const ASPECT_RATIO: f64 = 3.0 / 2.0;
    ///  let lookfrom = Point3::new(13.0, 2.0, 3.0);
    ///  let lookat = Point3::new(0.0, 0.0, 0.0);
    ///  let vup = Point3::new(0.0, 1.0, 0.0);
    ///  let dist_to_focus = 15.0;
    ///  let aperture = 0.1;
    ///     
    ///  let cam = Camera::new(
    ///      lookfrom,
    ///      lookat,
    ///      vup,
    ///      20.0,
    ///      ASPECT_RATIO,
    ///      aperture,
    ///      dist_to_focus,
    ///  );
    /// ```
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64, // Vertical field-of-view in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = common::degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
 
        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vec3::cross(vup, w));
        let v = vec3::cross(w, u);
 
        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
 
        let lens_radius = aperture / 2.0;
 
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }
 
    /// Generate a ray starting from a defined camera and going to the given coordinates.
    /// 
    /// # Exemple
    /// ```
    /// # use raytracer::{Point3, Vec3};
    /// use raytracer::Camera;
    /// 
    /// # const ASPECT_RATIO: f64 = 3.0 / 2.0;
    /// # let lookfrom = Point3::new(13.0, 2.0, 3.0);
    /// # let lookat = Point3::new(0.0, 0.0, 0.0);
    /// # let vup = Point3::new(0.0, 1.0, 0.0);
    /// # let dist_to_focus = 15.0;
    /// # let aperture = 0.1;
    /// let cam = Camera::new(
    ///     lookfrom,
    ///     lookat,
    ///     vup,
    ///     20.0,
    ///     ASPECT_RATIO,
    ///     aperture,
    ///     dist_to_focus,
    /// );
    /// 
    /// ```
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
 
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}