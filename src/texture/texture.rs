use crate::{Color,Point3};

pub trait Texture: Send + Sync {
    fn get_color(&self, u: f64, v:f64, point: &Point3) -> Color;
}