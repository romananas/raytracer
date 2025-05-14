use super::texture::Texture;
use crate::{Color,Point3};

pub struct SolidColor {
    albedo: Color
}

impl SolidColor {
    pub fn new(albedo: Color) -> SolidColor {
        SolidColor {
            albedo
        }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> SolidColor {
        let albedo = Color::new(red, green, blue);
        SolidColor {
            albedo
        }
    }
}

impl Texture for SolidColor {
    fn get_color(&self, _u: f64, _v:f64, _point: &Point3) -> Color {
        self.albedo
    }
}