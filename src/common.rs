// Constants
#![allow(unused_imports)]
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;
 
// Utility functions

/// Convert degrees to radians
/// 
/// # Exemples
/// 
/// ```
/// use raytracer::common::*;
/// 
/// assert_eq!(degrees_to_radians(180.0),PI);
/// assert_eq!(degrees_to_radians(360.0),2.0*PI);
/// ```
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    // Return a random real in [0.0, 1.0)
    rand::random()
}
 
pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Return a random real in [min, max)
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}