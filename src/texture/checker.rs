use super::texture::Texture;
use super::solid::SolidColor;
use crate::{Color,Point3};

/// Represents a 3D checkerboard texture that alternates between two sub-textures
/// based on the position in space.
///
/// Useful for creating surfaces like tiled floors, procedural patterns,
/// or grid-like materials with two different colors or textures.
pub struct CheckerTexture {
    /// Inverse of the scale factor that controls the size of the checker pattern.
    inv_scale: f64,
    /// Texture used for the "even" cells.
    even: Box<dyn Texture>,
    /// Texture used for the "odd" cells.
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    /// Constructs a new `CheckerTexture` using two textures and a scale factor.
    ///
    /// The scale defines the size of the checker cells. A smaller scale means larger cells.
    ///
    /// # Arguments
    /// * `scale` - Size scale of each checker cell.
    /// * `even` - Texture used for the even cells.
    /// * `odd` - Texture used for the odd cells.
    ///
    /// # Returns
    /// A new `CheckerTexture` instance.
    pub fn new(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    /// Convenience constructor that builds a checker texture using two solid colors.
    ///
    /// Internally wraps each color into a `SolidColor` texture.
    ///
    /// # Arguments
    /// * `scale` - Size scale of each checker cell.
    /// * `color1` - Color for the even cells.
    /// * `color2` - Color for the odd cells.
    ///
    /// # Returns
    /// A new `CheckerTexture` with two solid-colored cells.
    pub fn from_colors(scale: f64, color1: Color, color2: Color) -> CheckerTexture {
        let texture1 = SolidColor::new(color1);
        let texture2 = SolidColor::new(color2);

        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: Box::new(texture1),
            odd: Box::new(texture2),
        }
    }
}

impl Texture for CheckerTexture {
    /// Computes the texture color at a given point in space using a 3D checker pattern.
    ///
    /// Alternates between the `even` and `odd` textures based on the integer part of the
    /// scaled point coordinates.
    ///
    /// # Arguments
    /// * `u`, `v` - Texture coordinates (not used for pattern logic, but passed to sub-textures).
    /// * `point` - 3D point in space to sample the texture at.
    ///
    /// # Returns
    /// The color from either the `even` or `odd` texture based on the position.
    fn get_color(&self, u: f64, v: f64, point: &Point3) -> Color {
        let x = f64::floor(self.inv_scale * point.x()) as i32;
        let y = f64::floor(self.inv_scale * point.y()) as i32;
        let z = f64::floor(self.inv_scale * point.z()) as i32;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even.get_color(u, v, point)
        } else {
            self.odd.get_color(u, v, point)
        }
    }
}
