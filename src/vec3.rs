use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
 
use crate::common;

/// A 3D vector structure with three components (x, y, z).
/// This struct provides basic operations for 3D vector arithmetic.
///
/// # Examples
///
/// ```
/// let v = Vec3::new(1.0, 2.0, 3.0);
/// println!("{:?}", v);
/// ```
///
/// The struct implements `Copy`, `Clone`, `Default`, and `Debug` traits.
///
/// # Fields
///
/// * `e` - An array of three `f64` values representing the x, y, and z components.
#[derive(Debug,Copy, Clone, Default)]
pub struct Vec3 {
    e: [f64; 3],
}
 
impl Vec3 {
    /// Creates a new `Vec3` with the given x, y, and z values.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-component of the vector.
    /// * `y` - The y-component of the vector.
    /// * `z` - The z-component of the vector.
    ///
    /// # Returns
    ///
    /// A new `Vec3` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use raytracer::Vec3;
    /// 
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// assert_eq!(v.x(), 1.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
 
    pub fn random() -> Vec3 {
        Vec3::new(
            common::random_double(),
            common::random_double(),
            common::random_double(),
        )
    }
 
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            common::random_double_range(min, max),
            common::random_double_range(min, max),
            common::random_double_range(min, max),
        )
    }
 
    /// Returns the x-component of the vector.
    pub fn x(&self) -> f64 {
        self.e[0]
    }
 
    /// Returns the y-component of the vector.
    pub fn y(&self) -> f64 {
        self.e[1]
    }
 
    /// Returns the z-component of the vector.
    pub fn z(&self) -> f64 {
        self.e[2]
    }
 
    /// Returns the lenght of the vector
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
 
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
 
    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;
        // Return true if the vector is close to zero in all dimensions
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
    }
}
 
// Type alias
pub type Point3 = Vec3;
 
// Output formatting
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
 
/// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;
 
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}
 
/// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}
 
/// Vec3 *= f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}
 
/// Vec3 /= f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}
 
/// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;
 
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}
 
/// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;
 
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}
 
/// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}
 
/// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}
 
/// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
 
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}
 
/// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;
 
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}
 
/// Computes the dot product of two vectors.
/// 
/// The dot product is a measure of how much two vectors are pointing in the same direction.
/// Mathematically, it is calculated as:
/// 
/// `dot(u, v) = u_x * v_x + u_y * v_y + u_z * v_z`
/// 
/// **For beginners:** If two vectors are parallel, the dot product is maximized; if they are perpendicular, the dot product is zero.
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

/// Computes the cross product of two vectors.
/// 
/// The cross product results in a new vector that is perpendicular to both input vectors.
/// It is useful in 3D geometry for determining orientation and normals.
/// 
/// **For beginners:** The magnitude of the cross product represents the area of the parallelogram formed by the two vectors.
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

/// Returns the unit vector of a given vector.
/// 
/// A unit vector is a vector with length (magnitude) of 1, keeping the same direction.
/// It is computed by dividing the vector by its length.
/// 
/// **For beginners:** Unit vectors are useful for defining directions without considering magnitude.
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

/// Generates a random point inside a unit sphere.
/// 
/// This function repeatedly samples random points inside a cube until it finds one inside the unit sphere.
/// 
/// **For beginners:** This is useful for simulating random directions and diffusion effects.
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

/// Generates a random unit vector.
/// 
/// This function produces a random vector of length 1 by normalizing a randomly generated vector inside a sphere.
/// 
/// **For beginners:** This is often used in random sampling for physics simulations.
pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

/// Generates a random point inside a unit disk (in the XY plane).
/// 
/// Similar to `random_in_unit_sphere`, but constrained to a 2D disk.
/// 
/// **For beginners:** This is useful in optics and rendering, such as simulating depth-of-field effects.
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            common::random_double_range(-1.0, 1.0),
            common::random_double_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

/// Reflects a vector around a normal.
/// 
/// This is used in physics simulations, such as light reflection.
/// Mathematically, the reflection formula is:
/// 
/// `R = V - 2 * dot(V, N) * N`
/// 
/// **For beginners:** This is how light bounces off surfaces following the law of reflection.
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

/// Refracts a vector passing through a surface with a given refractive index ratio.
/// 
/// This models light bending when entering a different medium, using Snell’s Law.
/// The refracted vector consists of a perpendicular and a parallel component.
/// 
/// **For beginners:** This explains effects like a straw appearing bent in water.
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_parallel
}
