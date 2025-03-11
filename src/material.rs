use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::{common, vec3};

/// A record containing information about how a ray scatters after hitting a material.
pub struct ScatterRecord {
    /// The attenuation (color absorption) of the material.
    pub attenuation: Color,
    /// The scattered ray after interaction with the material.
    pub scattered: Ray,
}

/// A trait representing materials that can scatter light rays.
///
/// Implementors define how rays interact with the material surface.
pub trait Material: Send + Sync {
    /// Determines how a ray scatters after hitting a surface.
    ///
    /// # Arguments
    ///
    /// * `r_in` - The incoming ray.
    /// * `rec` - The hit record containing intersection details.
    ///
    /// # Returns
    ///
    /// An `Option<ScatterRecord>` describing the scattered ray and attenuation, or `None` if absorption occurs.
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

/// A Lambertian (diffuse) material that scatters light in random directions.
#[derive(Clone, Copy)]
pub struct Lambertian {
    /// The albedo (base color) of the material.
    albedo: Color,
}

impl Lambertian {
    /// Creates a new Lambertian material with the given albedo.
    ///
    /// # Arguments
    ///
    /// * `a` - The albedo (color) of the material.
    ///
    /// # Returns
    ///
    /// A new `Lambertian` material instance.
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, scatter_direction),
        })
    }
}

/// A metallic (reflective) material with optional fuzziness.
#[derive(Clone, Copy)]
pub struct Metal {
    /// The albedo (reflective color) of the metal.
    albedo: Color,
    /// The fuzziness factor (0 = perfect mirror, 1 = maximum blur).
    fuzz: f64,
}

impl Metal {
    /// Creates a new metallic material.
    ///
    /// # Arguments
    ///
    /// * `a` - The albedo (color) of the material.
    /// * `f` - The fuzziness of reflections (clamped between 0 and 1).
    ///
    /// # Returns
    ///
    /// A new `Metal` material instance.
    pub fn new(a: Color, f: f64) -> Metal {
        Metal {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere());

        if vec3::dot(scattered.direction(), rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

/// A dielectric (transparent/refractive) material that simulates glass-like behavior.
#[derive(Clone, Copy)]
pub struct Dielectric {
    /// The index of refraction of the material (e.g., 1.5 for glass).
    ir: f64,
}

impl Dielectric {
    /// Creates a new dielectric material.
    ///
    /// # Arguments
    ///
    /// * `index_of_refraction` - The refractive index of the material.
    ///
    /// # Returns
    ///
    /// A new `Dielectric` material instance.
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    /// Computes the reflectance using Schlick’s approximation.
    ///
    /// # Arguments
    ///
    /// * `cosine` - The cosine of the incident angle.
    /// * `ref_idx` - The index of refraction.
    ///
    /// # Returns
    ///
    /// The reflectance coefficient.
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = f64::min(vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > common::random_double()
        {
            vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        Some(ScatterRecord {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Ray::new(rec.p, direction),
        })
    }
}
