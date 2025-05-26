use super::texture::Texture;
use crate::{Color,Point3};

/// A simple texture that always returns a single solid color, regardless of the input.
///
/// Commonly used for uniform material colors or to serve as a base texture
/// for more complex procedural patterns.
pub struct SolidColor {
    /// The constant color returned by the texture.
    albedo: Color,
}

impl SolidColor {
    /// Creates a new `SolidColor` texture from a given `Color`.
    ///
    /// # Arguments
    /// * `albedo` - The solid color to be returned by the texture.
    ///
    /// # Returns
    /// A new `SolidColor` instance.
    pub fn new(albedo: Color) -> SolidColor {
        SolidColor { albedo }
    }

    /// Creates a new `SolidColor` from individual RGB components.
    ///
    /// # Arguments
    /// * `red` - Red channel (0.0 to 1.0).
    /// * `green` - Green channel (0.0 to 1.0).
    /// * `blue` - Blue channel (0.0 to 1.0).
    ///
    /// # Returns
    /// A new `SolidColor` instance with the specified RGB values.
    pub fn from_rgb(red: f64, green: f64, blue: f64) -> SolidColor {
        let albedo = Color::new(red, green, blue);
        SolidColor { albedo }
    }
}

impl Texture for SolidColor {
    /// Always returns the stored solid color, regardless of coordinates or position.
    ///
    /// # Arguments
    /// * `_u`, `_v` - Texture coordinates (ignored).
    /// * `_point` - 3D position in space (ignored).
    ///
    /// # Returns
    /// The constant color `albedo`.
    fn get_color(&self, _u: f64, _v: f64, _point: &Point3) -> Color {
        self.albedo
    }
}
